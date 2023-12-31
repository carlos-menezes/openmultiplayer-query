# openmultiplayer-query

![ci workflow](https://github.com/carlos-menezes/openmultiplayer-query/actions/workflows/ci.yml/badge.svg) ![Crates.io](https://img.shields.io/crates/v/openmultiplayer_query)

Implements the needed builders and parsers for SA:MP's/Open Multiplayer's [Query Mechanism](https://sampwiki.blast.hk/wiki/Query_Mechanism), allowing a developer to retrieve data from a running server.

## Installation

`cargo add openmultiplayer_query`

## Examples

Check `test/build.rs` and `test/parse.rs` to see how to use the library.

### Building

```rs
let mut packet = PacketBuilder::new(Opcode::I, Ipv4Addr::new(127, 0, 0, 1), 7777)?;
let mut rcon_packet = RconPacket::new(Ipv4Addr::new(127, 0, 0, 1), 7777, "changeme", "varlist")?;
```

### Parsing

```rs
use openmultiplayer_query::{Packet, Opcodes};

// Assume you have a UDP socket running
let socket = UdpSocket::bind("0.0.0.0:0")?;

// We'll send a packet to `149.56.84.18:7777`
let address: Ipv4Addr = "149.56.84.18".parse::<Ipv4Addr>().unwrap();
let port = 7777;

let mut packet = PacketBuilder::new(Opcodes::I, address, port)?;
// ...
packet.build()?; // This is needed in order to populate the data buffer with query data.

// Send the packet through the socket.
socket.send_to(packet.get_data().unwrap(), (address, port))?;
let mut recv_buf = [0u8; 2048];
socket.recv(&mut recv_buf)?;
let result: Result<Packet::InformationPacket, _> = (&recv_buf[..]).try_into();
// Use `result` as you please
```
