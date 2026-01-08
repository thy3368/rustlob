// SPDX-License-Identifier: GPL-2.0-or-later
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/tcp.h>
#include <linux/udp.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>

// 定义环形缓冲区用于 eBPF 到用户空间通信
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 8192);
} xdp_events SEC(".maps");

// 网络事件数据结构
struct xdp_event {
    __u64 timestamp;
    __u32 ifindex;
    __u32 protocol;
    __u32 src_ip;
    __u32 dst_ip;
    __u16 src_port;
    __u16 dst_port;
    __u32 pkt_len;
    __u8  eth_proto[2];
};

// 辅助函数：获取 IPv4 地址的字符串表示（用于调试）
static inline void ipv4_to_str(__u32 ip, char *buf, int len) {
    __u8 bytes[4];
    bytes[0] = (ip >> 24) & 0xFF;
    bytes[1] = (ip >> 16) & 0xFF;
    bytes[2] = (ip >> 8) & 0xFF;
    bytes[3] = ip & 0xFF;
    bpf_snprintf(buf, len, "%u.%u.%u.%u", bytes[0], bytes[1], bytes[2], bytes[3]);
}

SEC("xdp")
int xdp_hello(struct xdp_md *ctx)
{
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;

    // 解析以太网帧头部
    struct ethhdr *eth = data;
    if ((void *)eth + sizeof(*eth) > data_end)
        return XDP_PASS;

    // 解析 IPv4 数据包
    if (bpf_ntohs(eth->h_proto) == ETH_P_IP) {
        struct iphdr *iph = data + sizeof(*eth);
        if ((void *)iph + sizeof(*iph) > data_end)
            return XDP_PASS;

        // 解析传输层协议（TCP/UDP）
        __u16 src_port = 0;
        __u16 dst_port = 0;

        if (iph->protocol == IPPROTO_TCP) {
            struct tcphdr *tcph = (void *)iph + iph->ihl * 4;
            if ((void *)tcph + sizeof(*tcph) > data_end)
                return XDP_PASS;
            src_port = bpf_ntohs(tcph->source);
            dst_port = bpf_ntohs(tcph->dest);
        } else if (iph->protocol == IPPROTO_UDP) {
            struct udphdr *udph = (void *)iph + iph->ihl * 4;
            if ((void *)udph + sizeof(*udph) > data_end)
                return XDP_PASS;
            src_port = bpf_ntohs(udph->source);
            dst_port = bpf_ntohs(udph->dest);
        }

        // 分配事件空间并发送到用户空间
        struct xdp_event *event;
        event = bpf_ringbuf_reserve(&xdp_events, sizeof(*event), 0);
        if (event) {
            event->timestamp = bpf_ktime_get_ns();
            event->ifindex = ctx->ingress_ifindex;
            event->protocol = iph->protocol;
            event->src_ip = iph->saddr;
            event->dst_ip = iph->daddr;
            event->src_port = src_port;
            event->dst_port = dst_port;
            event->pkt_len = data_end - data;
            event->eth_proto[0] = eth->h_proto >> 8;
            event->eth_proto[1] = eth->h_proto & 0xFF;
            bpf_ringbuf_submit(event, 0);
        }
    }

    bpf_printk("Hello from XDP!");
    return XDP_PASS;
}

char LICENSE[] SEC("license") = "GPL";
uint32_t VERSION[] SEC("version") = { 0, 1, 0 };