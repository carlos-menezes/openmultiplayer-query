mod fake_data;
use openmultiplayer_query::packet;

#[cfg(test)]
mod tests {
    use crate::{
        fake_data::{
            CLIENT_LIST_PACKET, DETAILED_PLAYER_INFORMATION_PACKET, INFORMATION_PACKET,
            IS_OPEN_MULTIPLAYER_SERVER_PACKET, PING_PACKET, RULE_PACKET,
        },
        packet::{
            ClientListPacket, DetailedPlayerInformationPacket, InformationPacket,
            IsOpenMultiplayerServerPacket, PingPacket, RulePacket,
        },
    };

    #[test]
    fn test_send_i_packet() {
        let result: Result<InformationPacket, _> = (INFORMATION_PACKET).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_r_packet() {
        let result: Result<RulePacket, _> = (RULE_PACKET).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_c_packet() {
        let result: Result<ClientListPacket, _> = (CLIENT_LIST_PACKET).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_d_packet() {
        let result: Result<DetailedPlayerInformationPacket, _> =
            (DETAILED_PLAYER_INFORMATION_PACKET).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_o_packet() {
        let result: Result<IsOpenMultiplayerServerPacket, _> =
            (IS_OPEN_MULTIPLAYER_SERVER_PACKET).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_p_packet() {
        let result: Result<PingPacket, _> = (PING_PACKET).try_into();
        assert!(result.is_ok());
    }
}
