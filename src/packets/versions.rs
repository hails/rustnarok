use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PacketVersion {
    pub packets: HashMap<u16, PacketDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct PacketDefinition {
    pub packet: String,
    pub size: i16,
}

lazy_static! {
    pub static ref PACKET_VERSIONS: HashMap<u32, PacketVersion> = init();
}

fn init() -> HashMap<u32, PacketVersion> {
    let packet_db = std::fs::File::open("src/packets/packets.yml").unwrap();

    serde_yaml::from_reader(packet_db).unwrap()
}
