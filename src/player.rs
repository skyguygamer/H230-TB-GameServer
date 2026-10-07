/*
00000000000 0 0 00000 0000000 0000000
─────────── │ │ ───── ─────── ───────
 Player ID  │ │ Ammo  Stamina  Health
            │ └ Is healing
            └ Endianness
*/

pub struct Player {
    state: u32,
}

impl Player {
    pub fn new(player_id: u32) -> Player {
        let mut player: Player = Player { state: 0 };
        player.set_ammo(15);
        player.set_healing(false);
        player.set_health(75);
        player.set_endian(false);
        player.set_stamina(75);
        player.set_player_id(player_id);
        return player;
    }

    fn set_health(&mut self, health: u8) -> Result<u32, String> {
        let health_mask: u32 = 0b11111111111111111111111110000000;
        if health > 100 {
            return Err(String::from("Health cannot be above 100!")); //sends error if something is trying to set health greater than 100
        }
        let mut new_state: u32 = self.state & health_mask; //sets current health to 0

        new_state = new_state | health as u32;
        self.state = new_state;

        return Ok(new_state);
    }

    fn set_stamina(&mut self, stamina: u8) -> Result<u32, String> {
        let stamina_mask: u32 = 0b11111111111111111100000001111111;
        if stamina > 100 {
            return Err(String::from("Stamina cannot be above 100!")); //sends error if something is trying to set stamina greater than 100
        }

        let mut new_state: u32 = self.state & stamina_mask; //sets current stamina to 0
        let shifted_stamina: u32 = (stamina as u32) << 7;

        new_state = new_state | shifted_stamina;
        self.state = new_state;

        return Ok(new_state);
    }

    fn set_ammo(&mut self, ammo: u8) -> Result<u32, String> {
        let ammo_mask: u32 = 0b11111111111110000011111111111111;
        if ammo > 30 {
            return Err(String::from("Ammo cannot be above 30!")); //sends error if something is trying to set stamina greater than 100
        }

        let mut new_state: u32 = self.state & ammo_mask; //sets current ammo to 0
        let shifted_ammo: u32 = (ammo as u32) << 14;

        new_state = new_state | shifted_ammo;
        self.state = new_state;

        return Ok(new_state);
    }

    fn set_healing(&mut self, is_healing: bool) -> u32 {
        let healing_mask: u32 = 0b11111111111101111111111111111111;

        let mut new_state: u32 = self.state & healing_mask;
        let shifted_healing: u32 = (is_healing as u32) << 19;

        new_state = new_state | shifted_healing;
        self.state = new_state;

        return new_state;
    }

    fn set_endian(&mut self, is_big_endian: bool) -> u32 {
        let endian_mask: u32 = 0b11111111111011111111111111111111;
        //let endian_mask: u32 = !1048576; this works also because the
        //20th bit in the state is equal to 1,048,576 and using ! (not) flips it

        let mut new_state: u32 = self.state & endian_mask;
        let shifted_endian: u32 = (is_big_endian as u32) << 20;

        new_state = new_state | shifted_endian;
        self.state = new_state;

        return new_state;
    }

    //Change this later so it checks with the server to make sure that ID is available and not being used
    fn set_player_id(&mut self, player_id: u32) -> Result<u32, String> {
        let player_id_mask: u32 = 0b00000000001111111111111111111111;
        if player_id > 2047 {
            return Err(String::from("Player ID can only be below 2048!")); //sends error if something is trying to set player ID greater than 2047
        }

        let mut new_state: u32 = self.state & player_id_mask; //sets player id to 0
        let shifted_player_id: u32 = (player_id as u32) << 21;

        new_state = new_state | shifted_player_id;
        self.state = new_state;

        return Ok(new_state);
    }

    pub fn is_healing(&self) -> bool {
        return ((self.state >> 19) & 1) == 1;
    }

    pub fn get_endian(&self) -> bool {
        return ((self.state >> 20) & 1) == 1;
    }

    pub fn get_health(&self) -> u8 {
        let health_mask: u32 = 0b1111111;

        let health: u32 = self.state & health_mask;

        return health as u8;
    }

    pub fn get_stamina(&self) -> u8 {
        let shifted: u32 = self.state >> 7;
        let stamina_mask: u32 = 0b1111111;

        let masked: u32 = shifted & stamina_mask;

        return masked as u8;
    }

    pub fn get_ammo(&self) -> u8 {
        let shifted: u32 = self.state >> 14;
        let ammo_mask: u32 = 31; //Using decimal or binary works here to set masks

        let masked: u32 = shifted & ammo_mask;

        return masked as u8;
    }

    pub fn get_player_id(&self) -> u16 {
        let shifted: u32 = self.state >> 21;
        let masked: u32 = shifted & 2047;

        return masked as u16;
    }

    // Returns the Player ID as two bytes in big-endian order.
    // Uses the endian flag to determine if conversion is needed.
    pub fn get_player_id_big_endian(&self) -> [u8; 2] {
        let id: u16 = self.get_player_id();

        if !self.get_endian() {
            // Endian bit says little-endian, so convert to big-endian bytes
            return id.to_be_bytes();
        } else {
            // Endian bit says big-endian, so represent it as big-endian bytes
            return [(id >> 8) as u8, id as u8];
        }
    }
}
