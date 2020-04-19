use nom::number::complete::{le_u16, le_u32, le_u8};
use std::str::from_utf8;

pub trait IncomingPacket {
    type Output;
    fn read(buf: &[u8]) -> Self::Output;
}

#[derive(Debug, Default, PartialEq)]
pub struct CALoginPacket {
    pub id: u16,
    pub version: u32,
    pub username: String,
    pub password: String,
    pub client_type: u8,
}

impl IncomingPacket for CALoginPacket {
    type Output = Self;

    fn read(buf: &[u8]) -> Self {
        debug!("Raw pkt: {:?}", buf);
        debug!("pkg len: {}", buf.len());
        named!(string_24<&str>, map_res!(take!(24), from_utf8));
        named!(
            parse_ca_login_packet<CALoginPacket>,
            do_parse!(
                // id: le_u16
                //     >>
                version: le_u32
                    >> username: string_24
                    >> password: string_24
                    >> client_type: le_u8
                    >> (CALoginPacket {
                        id: 999,
                        version: version,
                        username: username.to_string().replace("\u{0}", ""),
                        password: password.to_string().replace("\u{0}", ""),
                        client_type: client_type
                    })
            )
        );
        let parsed = parse_ca_login_packet(buf).unwrap();
        debug!("Leftover: {:x?}", parsed.0);

        parsed.1
    }

    // fn new(&mut self, buf: &[u8]) {
    //     let mut username = [0u8; 24];
    //     let mut password = [0u8; 24];

    //     self.header = buf.read_u16::<LittleEndian>().unwrap();
    //     self.version = buf.read_u32::<LittleEndian>().unwrap();

    //     buf.read_exact(&mut username).unwrap();
    //     buf.read_exact(&mut password).unwrap();

    //     self.client_type = buf.read_u8().unwrap();

    //     self.username = std::str::from_utf8(&username[..]).unwrap().to_string();
    //     self.password = std::str::from_utf8(&password[..]).unwrap().to_string();
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ca_login() {
        let received_packet = [
            100, 0, 25, 0, 0, 0, 116, 101, 115, 116, 105, 110, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 116, 101, 115, 116, 105, 110, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 12,
        ];

        let expected = CALoginPacket {
            id: 100,
            version: 25,
            username: "testing".to_string(),
            password: "testing".to_string(),
            client_type: 12,
        };

        assert_eq!(CALoginPacket::read(&received_packet), expected);
    }
}
