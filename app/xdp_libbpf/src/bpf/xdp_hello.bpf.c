// SPDX-License-Identifier: GPL-2.0-or-later

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_endian.h>

#define ETH_P_IP    0x0800

// 定义环形缓冲区用于 eBPF 到用户空间通信
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 8192);
} xdp_events SEC(".maps");

// 网络事件数据结构
struct xdp_event {
    u64 timestamp;
    u32 ifindex;
    u32 protocol;
    u32 src_ip;
    u32 dst_ip;
    u16 src_port;
    u16 dst_port;
    u32 pkt_len;
    u8  eth_proto[2];
};

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
        u16 src_port = 0;
        u16 dst_port = 0;

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

char _license[] SEC("license") = "GPL";