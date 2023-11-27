pub mod errors;
pub mod opcodes;
pub mod packet;

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, UdpSocket};

    use crate::{
        errors::PacketError,
        opcodes::Opcode,
        packet::{
            ClientListPacket, DetailedPlayerInformationPacket, InformationPacket,
            IsOpenMultiplayerServerPacket, Packet, PingPacket, RulePacket,
        },
    };

    fn send_packet(opcode: Opcode) -> Result<[u8; 2048], PacketError> {
        let address: Ipv4Addr = "149.56.84.18".parse::<Ipv4Addr>().unwrap();
        let port = 7777;
        let mut packet = Packet::new(opcode, address, port)?;
        packet.build()?;
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        println!("socket: {}", socket.local_addr().unwrap().port());
        socket.send_to(packet.get_data().unwrap(), (address, port))?;
        let mut recv_buf = [0u8; 2048];
        socket.recv(&mut recv_buf)?;
        Ok(recv_buf)
    }

    #[test]
    fn test_send_i_packet() {
        let recv_buf = send_packet(Opcode::I).unwrap();
        let result: Result<InformationPacket, _> = (&recv_buf[..]).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_r_packet() {
        let recv_buf = send_packet(Opcode::R).unwrap();
        let result: Result<RulePacket, _> = (&recv_buf[..]).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_c_packet() {
        let recv_buf = send_packet(Opcode::C).unwrap();
        let result: Result<ClientListPacket, _> = (&recv_buf[..]).try_into();
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    fn test_send_d_packet() {
        let recv_buf = send_packet(Opcode::D).unwrap();
        let result: Result<DetailedPlayerInformationPacket, _> = (&recv_buf[..]).try_into();
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    fn test_send_o_packet() {
        let recv_buf = send_packet(Opcode::O).unwrap();
        let result: Result<IsOpenMultiplayerServerPacket, _> = (&recv_buf[..]).try_into();
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    fn test_send_p_packet() {
        let recv_buf = send_packet(Opcode::P).unwrap();
        let result: Result<PingPacket, _> = (&recv_buf[..]).try_into();
        assert!(result.is_ok());
    }
}
