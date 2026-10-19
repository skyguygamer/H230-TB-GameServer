use crate::player::Player;


// 'a is a lifetime parameter that tells Rust how long
// the borrowed Player reference must remain valid for
pub struct PlayerSession<'a> {
    //Borrows the player without taking ownership
    player: &'a Player,
    session_id: u32,
    is_connected: bool,
    ipv4_address: u32,
}

// 'a connects the lifetime of the sessions player reference
// to the Player passed into the constructor
impl<'a> PlayerSession<'a> {

    //Creates a session that references an existing player
    pub fn new(player: &'a Player, session_id: u32, ipv4_address: u32) -> PlayerSession<'a> {
        PlayerSession {
            player: player,
            session_id: session_id,
            is_connected: true,
            ipv4_address: ipv4_address,
        }
    }

    // Reads information from the session and its borrowed Player.
    // &self means we don't modify or take ownership of the session
    pub fn print_info(&self) {
        println!("Player ID: {}", self.player.get_player_id());
        println!("Session ID: {}", self.session_id);
        println!("Connected: {}", self.is_connected);
        println!("IPv4 Address: {}", self.ipv4_address as u32);
    }
}

//Borrows a PlayerSession that itself borrows a Player.
// The lifetime parameter describes the reference stored in the session
pub fn display_session<'a>(session: &PlayerSession<'a>) {
    session.print_info();
}




//'a is a lifetime parameter. It tells Rust that my PlayerSession contains a reference to a Player. 
//Rust uses this relationship to make sure the Player remains valid whenever the session accesses it. 
//It doesn't extend the Player's lifetime; it helps the compiler prevent invalid references.