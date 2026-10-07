//Topic 2
/*
00000000000 0 0 00000 0000000 0000000
─────────── │ │ ───── ─────── ───────
 Player ID  │ │ Ammo  Stamina  Health
            │ └ Is healing
            └ Endianness
*/

mod player;
use player::Player;

fn main() {
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
}