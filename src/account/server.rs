use crate::packets::incoming::{CALoginPacket, IncomingPacket};
use crate::packets::versions::PACKET_VERSIONS;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};

pub fn run() {
    let listener = TcpListener::bind("127.1:6900").unwrap();
    info!("Account server is listening");
    for stream in listener.incoming() {
        handle_raw_packet(&mut stream.unwrap());
    }
}

fn handle_raw_packet(stream: &mut TcpStream) {
    stream.set_nodelay(true).expect("Failed to set_nodelay");

    let id = stream.read_u16::<LittleEndian>().unwrap();
    let buf_size: u16;

    if let Some(packet_ver) = PACKET_VERSIONS.get(&20151104) {
        if let Some(packet) = packet_ver.packets.get(&id) {
            if packet.size > 0 {
                buf_size = packet.size as u16 - 2;
            } else {
                buf_size = stream.read_u16::<LittleEndian>().unwrap() - 2;
            }
        } else {
            error!("Invalid packet id: {}", id);
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }
    } else {
        error!("Invalid packetver");
        stream.shutdown(Shutdown::Both).unwrap();
        return;
    }

    let mut buf = vec![0u8; buf_size as usize];
    stream.read(&mut buf).unwrap();

    // TODO Put id and size back into the buffer
    let ca = CALoginPacket::read(&buf);
    debug!("Received packet: \n{:#?}", ca);

    return;
}
