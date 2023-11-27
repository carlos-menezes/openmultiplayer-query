use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek},
    net::{IpAddr, Ipv4Addr},
};

use byteorder::{LittleEndian, ReadBytesExt};
use encoding_rs::WINDOWS_1251;

use crate::{errors::PacketError, opcodes::Opcode};

#[derive(Debug)]
pub struct Packet {
    pub opcode: Opcode,
    pub address: Ipv4Addr,
    pub port: u16,
    data: Option<[u8; 11]>,
}

#[derive(Debug)]
pub struct InformationPacket {
    pub password: bool,
    pub players: u16,
    pub max_players: u16,
    pub hostname: String,
    pub gamemode: String,
    pub language: String,
}

#[derive(Debug)]
pub struct RulePacket {
    pub rules: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Player {
    pub nickname: String,
    pub score: u32,
}

#[derive(Debug)]
pub struct ClientListPacket {
    pub players: Vec<Player>,
}

#[derive(Debug)]
pub struct DetailedPlayer {
    pub id: u8,
    pub nickname: String,
    pub score: u32,
    pub ping: u32,
}

#[derive(Debug)]
pub struct DetailedPlayerInformationPacket {
    pub players: Vec<DetailedPlayer>,
}

#[derive(Debug)]
pub struct IsOpenMultiplayerServerPacket(pub bool);

#[derive(Debug)]
pub struct PingPacket(pub [u8; 4]);

impl Packet {
    pub fn new<T: Into<IpAddr>>(
        opcode: Opcode,
        address: T,
        port: u16,
    ) -> Result<Packet, PacketError> {
        let addr = address.into();
        match addr {
            IpAddr::V4(ipv4_addr) => Ok(Self {
                opcode,
                address: ipv4_addr,
                port,
                data: None,
            }),
            _ => Err(PacketError::InvalidAddress(addr.to_string())),
        }
    }

    pub fn build(&mut self) -> Result<(), PacketError> {
        let mut data = [0u8; 11];

        // Fill 'SAMP'
        data[0..4].copy_from_slice(b"SAMP");

        // Fill IP address
        data[4..8].copy_from_slice(&self.address.octets());

        // Fill port
        data[8] = (self.port & 0xFF) as u8;
        data[9] = (self.port >> 8) as u8;

        // Fill opcode
        data[10] = self.opcode.into();
        self.data = Some(data);
        Ok(())
    }

    pub fn get_data(&self) -> Result<&[u8; 11], PacketError> {
        self.data.as_ref().ok_or(PacketError::PacketNotBuilt)
    }
}

impl TryFrom<&[u8]> for InformationPacket {
    type Error = PacketError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(std::io::SeekFrom::Current(11)).unwrap();

        let password = cursor.read_u8()? == 1;
        let players = cursor.read_u16::<LittleEndian>()?;
        let max_players = cursor.read_u16::<LittleEndian>()?;

        let hostname_len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut hostname = vec![0; hostname_len];
        cursor.read_exact(&mut hostname)?;
        let (decoded_string, _, had_errors) = WINDOWS_1251.decode(&hostname);
        if had_errors {
            return Err(PacketError::Windows1251Error);
        }
        let hostname = decoded_string.into_owned();

        let gamemode_len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut gamemode = String::new();
        cursor
            .by_ref()
            .take(gamemode_len as u64)
            .read_to_string(&mut gamemode)?;

        let language_len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut language = vec![0; language_len];
        cursor.read_exact(&mut language)?;
        let (decoded_string, _, had_errors) = WINDOWS_1251.decode(&language);
        if had_errors {
            return Err(PacketError::Windows1251Error);
        }
        let language = decoded_string.into_owned();

        Ok(Self {
            password,
            players,
            max_players,
            hostname,
            gamemode,
            language,
        })
    }
}

impl TryFrom<&[u8]> for RulePacket {
    type Error = PacketError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(std::io::SeekFrom::Current(11))?;

        let rule_count = cursor.read_u16::<LittleEndian>()?;
        let mut rules = HashMap::new();

        for _ in 0..rule_count {
            let rule_name_len = cursor.read_u8()? as usize;
            let mut rule_name = String::new();
            cursor
                .by_ref()
                .take(rule_name_len as u64)
                .read_to_string(&mut rule_name)?;

            let rule_value_len = cursor.read_u8()? as usize;
            let mut rule_value = String::new();
            cursor
                .by_ref()
                .take(rule_value_len as u64)
                .read_to_string(&mut rule_value)?;

            rules.insert(rule_name, rule_value);
        }

        Ok(RulePacket { rules })
    }
}

impl TryFrom<&[u8]> for ClientListPacket {
    type Error = PacketError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(std::io::SeekFrom::Current(11))?;

        let player_count = cursor.read_u16::<LittleEndian>()?;
        let mut players = Vec::new();

        for _ in 0..player_count {
            let nickname_len = cursor.read_u8()? as usize;
            let mut nickname = vec![0; nickname_len];
            cursor.read_exact(&mut nickname)?;
            let nickname = String::from_utf8(nickname)?;

            let score = cursor.read_u32::<LittleEndian>()?;

            players.push(Player { nickname, score });
        }

        Ok(ClientListPacket { players })
    }
}

impl TryFrom<&[u8]> for DetailedPlayerInformationPacket {
    type Error = PacketError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(std::io::SeekFrom::Current(11))?;

        let player_count = cursor.read_u16::<LittleEndian>()?;
        let mut players = Vec::new();

        for _ in 0..player_count {
            let id = cursor.read_u8()?;
            let nickname_len = cursor.read_u8()? as usize;
            let mut nickname = vec![0; nickname_len];
            cursor.read_exact(&mut nickname)?;
            let nickname = String::from_utf8(nickname)?;

            let score = cursor.read_u32::<LittleEndian>()?;
            let ping = cursor.read_u32::<LittleEndian>()?;

            players.push(DetailedPlayer {
                id,
                nickname,
                score,
                ping,
            });
        }

        Ok(DetailedPlayerInformationPacket { players })
    }
}

impl TryFrom<&[u8]> for IsOpenMultiplayerServerPacket {
    type Error = PacketError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(std::io::SeekFrom::Current(11))?;

        let is_open_multiplayer_server = cursor.read_u8()? == 1;

        Ok(IsOpenMultiplayerServerPacket(is_open_multiplayer_server))
    }
}

impl TryFrom<&[u8]> for PingPacket {
    type Error = PacketError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(value);
        cursor.seek(std::io::SeekFrom::Current(11))?;

        let mut ping = [0u8; 4];
        cursor.read_exact(&mut ping)?;

        Ok(PingPacket(ping))
    }
}
