#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    helpers::bpf_ktime_get_ns,
    macros::map,
    maps::RingBuf,
    programs::XdpContext,
};
use aya_log_ebpf::info;
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};

// 网络事件数据结构（与用户空间程序中的结构匹配）
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XdpEvent {
    pub timestamp: u64,
    pub ifindex: u32,
    pub protocol: u32,
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub pkt_len: u32,
    pub eth_proto: u16,
}

// 环形缓冲区用于向用户空间发送事件
#[map(name = "xdp_events")]
static mut XDP_EVENTS: RingBuf = RingBuf::with_byte_size(1024 * 1024, 0);

// XDP 程序
pub fn xdp_hello(ctx: XdpContext) -> u32 {
    match try_xdp_hello(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_PASS,
    }
}

fn try_xdp_hello(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = ctx.data() as *const EthHdr;
    if ethhdr.is_null() {
        return Ok(xdp_action::XDP_PASS);
    }

    let hdr_size = mem::size_of::<EthHdr>();
    let data_end = ctx.data_end();
    if (ethhdr as usize + hdr_size) > data_end {
        return Ok(xdp_action::XDP_PASS);
    }

    // 解析以太网帧
    let ethhdr = unsafe { &*ethhdr };

    if ethhdr.ether_type == EtherType::Ipv4 as u16 {
        // IPv4 数据包处理
        handle_ipv4(ctx, ethhdr)?;
    } else {
        // 其他协议直接通过
        return Ok(xdp_action::XDP_PASS);
    }

    Ok(xdp_action::XDP_PASS)
}

fn handle_ipv4(ctx: XdpContext, ethhdr: &EthHdr) -> Result<(), ()> {
    let iphdr: *const Ipv4Hdr = (ctx.data() as usize + mem::size_of::<EthHdr>()) as *const Ipv4Hdr;

    let hdr_size = mem::size_of::<EthHdr>() + mem::size_of::<Ipv4Hdr>();
    let data_end = ctx.data_end();
    if (iphdr as usize + mem::size_of::<Ipv4Hdr>()) > data_end {
        return Err(());
    }

    let iphdr = unsafe { &*iphdr };

    // 解析传输层协议
    let protocol = IpProto::from(iphdr.proto);
    let src_ip = u32::from_be_bytes(iphdr.src_addr);
    let dst_ip = u32::from_be_bytes(iphdr.dst_addr);

    let mut src_port = 0u16;
    let mut dst_port = 0u16;

    let transport_offset = mem::size_of::<EthHdr>() + (iphdr.ihl() as usize * 4);
    if protocol == IpProto::Tcp {
        // TCP
        let tcphdr: *const TcpHdr = transport_offset as *const TcpHdr;
        if (tcphdr as usize + mem::size_of::<TcpHdr>()) > data_end {
            return Err(());
        }
        let tcphdr = unsafe { &*tcphdr };
        src_port = u16::from_be_bytes(tcphdr.source);
        dst_port = u16::from_be_bytes(tcphdr.dest);
    } else if protocol == IpProto::Udp {
        // UDP
        let udphdr: *const UdpHdr = transport_offset as *const UdpHdr;
        if (udphdr as usize + mem::size_of::<UdpHdr>()) > data_end {
            return Err(());
        }
        let udphdr = unsafe { &*udphdr };
        src_port = u16::from_be_bytes(udphdr.src);
        dst_port = u16::from_be_bytes(udphdr.dst);
    }

    // 创建事件
    let event = XdpEvent {
        timestamp: bpf_ktime_get_ns(),
        ifindex: ctx.metadata() as u32,
        protocol: protocol as u32,
        src_ip,
        dst_ip,
        src_port,
        dst_port,
        pkt_len: (ctx.data_end() - ctx.data()) as u32,
        eth_proto: ethhdr.ether_type,
    };

    // 发送到环形缓冲区
    unsafe {
        if let Some(mut entry) = XDP_EVENTS.reserve::<XdpEvent>(0) {
            entry.write(event);
            entry.submit(0);
        }
    }

    info!(
        &ctx,
        "IPv4 packet: {}:{} -> {}:{}",
        src_ip, src_port, dst_ip, dst_port
    );

    Ok(())
}

// 程序入口点
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
