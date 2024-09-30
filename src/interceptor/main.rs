use std::io::ErrorKind;
use std::time::Duration;
use pnet::datalink::{interfaces, Channel, channel, Config, DataLinkReceiver, NetworkInterface};
use sudo::{check, escalate_if_needed, RunningAs};
use pnet_packet::Packet;
use pnet_packet::icmp::IcmpPacket;
use pnet_packet::ipv4;
use serde_json::{Map, Value};
use byteorder::{ByteOrder, BigEndian};

fn get_type(packet: &[u8]) -> u8 {
    packet[0..1][0]
}

fn get_code(packet: &[u8]) -> u8 {
    packet[1..2][0]
}

fn get_checksum(packet: &[u8]) -> u16 {
    let checksum: &[u8; 2] = &packet[2..4].try_into().unwrap();
    BigEndian::read_u16(checksum)
}

fn get_id(packet: &[u8]) -> u16 {
    let id: &[u8; 2] = &packet[4..6].try_into().unwrap();
    BigEndian::read_u16(id)
}

fn get_sequence_number(packet: &[u8]) -> u16 {
    let seq = (&packet[6..8]).try_into().unwrap();
    BigEndian::read_u16(seq)
}

fn craft_json(packet: IcmpPacket) -> Value {
    let mut obj = Map::new();
    obj.insert("type".into(), Value::String(get_type(packet.packet()).to_string()));
    obj.insert("code".into(), Value::String(get_code(packet.packet()).to_string()));
    obj.insert("checksum".into(), Value::String(packet.get_checksum().to_string()));
    obj.insert("id".into(), Value::String(get_id(packet.packet()).to_string()));
    obj.insert("sequence".into(), Value::String(get_sequence_number(packet.packet()).to_string()));
    obj.insert("data".into(), Value::String(String::from_utf8_lossy(&packet.packet()[8..]).to_string()));
    Value::Object(obj)
}

fn main() -> () {
    match check() {
        RunningAs::Root => {},
        RunningAs::Suid => {},
        RunningAs::User => { escalate_if_needed().expect("Error getting sudo user"); }
    }
    let config: Config = Config {
        read_timeout: Some(Duration::new(1, 0)),
        ..Default::default()
    };

    let mut receivers: Vec<(NetworkInterface,Box<dyn DataLinkReceiver>)> = vec![];
    let channels: Vec<(NetworkInterface, Channel)> =
        interfaces().into_iter().map(|iface| (iface.clone(), channel(&iface, config).unwrap())).collect();
    for c in channels {
        if let Channel::Ethernet(_, rx) = c.1 {
            receivers.push((c.0, rx));
        }
    }

    for mut r in receivers {
        // println!("interface: {}", r.0.name);
        let mut tries = 0;
        loop {
            match r.1.next() {
                Ok(packet) => {
                    if let Some(eth_packet) = pnet_packet::ethernet::EthernetPacket::new(packet) {
                        if let Some(ipv4_packet) = ipv4::Ipv4Packet::new(eth_packet.payload()) {
                            if ipv4_packet.get_next_level_protocol() == pnet_packet::ip::IpNextHeaderProtocols::Icmp {
                                if let Some(icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
                                    println!("{}", craft_json(icmp_packet));
                                } else {
                                    // println!("Cannot create ICMP packet");
                                }
                            } else {
                                // println!("Not an ICMP packet");
                            }
                        } else {
                            // println!("Not an IPv4 packet");
                        }
                    } else {
                        // println!("Cannot extract ethernet frame");
                    }
                }
                Err(e) => {
                    if e.kind() == ErrorKind::TimedOut {
                        eprintln!("No packets found");
                    } else {
                        eprintln!("{}", e);
                    }
                },
            }
            tries += 1;
            if tries == 10 {
                break;
            }
        }
        println!();
    }
}

