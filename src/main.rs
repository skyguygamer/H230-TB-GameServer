//Topic 2
/*
00000000000 0 0 00000 0000000 0000000
─────────── │ │ ───── ─────── ───────
 Player ID  │ │ Ammo  Stamina  Health
            │ └ Is healing
            └ Endianness
*/

mod player;
mod session;
use player::Player;
use session::PlayerSession;

fn main() {
    // Creates the Player in the outer scope
    let player = Player::new(1);

    println!("--- Player State Test ---");
    println!("Player ID: {}", player.get_player_id());
    println!("Health: {}", player.get_health());
    println!("Stamina: {}", player.get_stamina());
    println!("Ammo: {}", player.get_ammo());
    println!("Is Healing: {}", player.is_healing());
    println!("Is Big Endian: {}", player.get_endian());

    println!("\n--- Endian Test ---");
    println!("Normal Player ID: {}", player.get_player_id());
    println!(
        "Player ID as big-endian bytes: {:?}",
        player.get_player_id_big_endian()
    );

    println!("\n--- Session Test ---");
    {
        //Borrows the player using &player instead of transferring ownership
        let session = PlayerSession::new(&player, 100, 0xC0A80101);

        //Pass the session by reference to a function with a lifetime parameter
        session::display_session(&session);
    } //Session gets dropped here
    //This statement below proves that Player is still accessible after the session ends
    println!("Player ID: {}", player.get_player_id());
}
