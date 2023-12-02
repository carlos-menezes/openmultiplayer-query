#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use openmultiplayer_query::{
        opcodes::Opcode,
        packet::{Packet, PacketBuilder, RconPacket},
    };

    #[test]
    fn test_build_packet() {
        let mut packet = PacketBuilder::new(Opcode::I, Ipv4Addr::new(127, 0, 0, 1), 7777).unwrap();

        assert!(packet.build().is_ok());

        let expected_data = [
            0x53, 0x41, 0x4D, 0x50, 0x7F, 0x00, 0x00, 0x01, 0x61, 0x1E, 0x69,
        ];

        assert_eq!(*packet.get_data().unwrap(), expected_data);
    }

    #[test]
    fn test_build_rcon_packet() {
        let mut packet =
            RconPacket::new(Ipv4Addr::new(127, 0, 0, 1), 7777, "changeme", "varlist").unwrap();

        assert!(packet.build().is_ok());

        let expected_data = vec![
            0x08, 0x00, 0x63, 0x68, 0x61, 0x6E, 0x67, 0x65, 0x6D, 0x65, 0x07, 0x00, 0x76, 0x61,
            0x72, 0x6C, 0x69, 0x73, 0x74,
        ];

        assert_eq!(*packet.get_data().unwrap(), expected_data);
    }
}
