use std::io::ErrorKind;
use std::time::Duration;
use pnet::datalink::{interfaces, Channel, channel, Config, DataLinkReceiver};
use sudo::{check, escalate_if_needed, RunningAs};
use pnet_packet::{icmp, Packet};
use pnet_packet::ipv4;


fn main() -> () {
    match check() {
        RunningAs::Root => {},
        RunningAs::Suid => {},
        RunningAs::User => { escalate_if_needed().expect("Error getting sudo user"); }
    }
    let config: Config = Config {
        read_timeout: Some(Duration::new(1, 0)),
        // bpf_fd_attempts: 10,
        ..Default::default()
    };
    // let mut receivers: Vec<&mut Box<dyn DataLinkReceiver>> =
    //     interfaces().iter().map(|iface| channel(iface, config)).collect::<Result<Channel>>().filter(|channel| match channel {
    //         Ok(Channel::Ethernet(_, mut rx)) => true,
    //         _ => false
    //     }).collect();
    // let _ = interfaces().iter().map(|iface| {
    //     let r = channel(iface, config);
    //     match r {
    //         Ok(Channel::Ethernet(_, mut rx)) => {
    //             receivers.push(&mut rx)
    //         },
    //         _ => {},
    //     }
    // });
    // for r in receivers {
    //     match r.next() {
    //         Ok(packet) => {
    //             if let Some(eth_packet) = pnet_packet::ethernet::EthernetPacket::new(packet) {
    //                 if let Some(ipv4_packet) = ipv4::Ipv4Packet::new(eth_packet.payload()) {
    //                     if ipv4_packet.get_next_level_protocol() == pnet_packet::ip::IpNextHeaderProtocols::Icmp {
    //                         if let Some(icmp_packet) = icmp::IcmpPacket::new(ipv4_packet.payload()) {
    //                             println!("Packet found:");
    //                             println!("ICMP Type: {}", icmp_packet.get_icmp_type().0);
    //                         } else {
    //                             println!("Cannot create ICMP packet");
    //                         }
    //                     } else {
    //                         println!("Not an ICMP packet");
    //                     }
    //                 } else {
    //                     println!("Not an IPv4 packet");
    //                 }
    //             } else {
    //                 println!("Cannot extract ethernet frame");
    //             }
    //         }
    //         Err(e) => {
    //             if e.kind() == ErrorKind::TimedOut {
    //                 eprintln!("No packets found");
    //             } else {
    //                 eprintln!("{}", e);
    //             }
    //         },
    //     }
    //     println!();
    // }
    // for i in &ifaces {
        // println!("{}", i.name);
        // match channel(&i, config) {
        //     Ok(Channel::Ethernet(_, mut rx)) =>
        //         match rx.next() {
        //             Ok(packet) => {
        //                 if let Some(eth_packet) = pnet_packet::ethernet::EthernetPacket::new(packet) {
        //                     if let Some(ipv4_packet) = ipv4::Ipv4Packet::new(eth_packet.payload()) {
        //                         if ipv4_packet.get_next_level_protocol() == pnet_packet::ip::IpNextHeaderProtocols::Icmp {
        //                             if let Some(icmp_packet) = icmp::IcmpPacket::new(ipv4_packet.payload()) {
        //                                 println!("Packet found:");
        //                                 println!("ICMP Type: {}", icmp_packet.get_icmp_type().0);
        //                             } else {
        //                                 println!("Cannot create ICMP packet");
        //                             }
        //                         } else {
        //                             println!("Not an ICMP packet");
        //                         }
        //                     } else {
        //                         println!("Not an IPv4 packet");
        //                     }
        //                 } else {
        //                     println!("Cannot extract ethernet frame");
        //                 }
        //             }
        //             Err(e) => {
        //                 if e.kind() == ErrorKind::TimedOut {
        //                     eprintln!("No packets found");
        //                 } else {
        //                     eprintln!("{}", e);
        //                 }
        //             },
        //         },
        //     Err(e) => {
        //         if e.kind() == ErrorKind::TimedOut {
        //             eprintln!("Interface not active");
        //         } else {
        //             eprintln!("{}", e);
        //         }
        //     },
        //     Ok(_) => eprintln!("Other"),
        // }
        // println!("");
    // }
}

