#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    I, // This stands for information. This gets the amount of players in the server, the map name, and all the stuff like that. It's really useful for describing your server without changing anything.
    R, // This stands for rules. 'Rules' when it comes to SA:MP includes the instagib, the gravity, weather, the website URL, and so on.
    C, // It stands for client list, this sends back to the server the players' name, and then the players' score. Just imagine it as a basic overview of all the players.
    D, // This stands for detailed player information. With this, you can get everything from the ping to the player, the player ID (useful for admin scripts), the score again, and also the username.
    X, // This is an RCON command, and it's completely different from all of the other packets.
    P, // Four pseudo-random characters are sent to the server, and the same characters are returned. You can use the time between sending and receiving to work out the servers' ping/latency.
    O, // Checks whether the server is using open.mp or not.
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::I => b'i',
            Opcode::R => b'r',
            Opcode::C => b'c',
            Opcode::D => b'd',
            Opcode::X => b'x',
            Opcode::P => b'p',
            Opcode::O => b'o',
        }
    }
}
