use num_traits::FromPrimitive;
use num_derive::{ FromPrimitive, ToPrimitive };
use crate::api_stream::Stream;
use crate::packet::{Packet, PacketData};
use uuid::Uuid;
use rand::Rng;

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum IncomingTypes {
    LoginStart = 0x00
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum OutgoingTypes {
    KeepAlive = 0x0E,
    JoinGame = 0x25
}

#[derive(Debug)]
struct KeepAliveData {
    id: i64
}

impl KeepAliveData {
    pub fn new() -> KeepAliveData {
        KeepAliveData { id: rand::thread_rng().gen() }
    }
}

impl PacketData for KeepAliveData {
    fn transmit(&self, stream: &mut Stream) {
        stream.write_long(self.id);
    }
}

#[derive(Debug)]
struct JoinGameData {
    entity_id: i32,
    game_mode: u8,   // 0, 1, 2, 3 - Survival, Creative, Adventure, Spectator (bit 3 = hardcore flag)
    dimension: i32,  // -1, 0, 1 - Nether, Overworld, End
    difficulty: u8,  // 0, 1, 2, 3 - Peaceful, Easy, Normal, Hard
    max_players: u8, // Ignored
    level_type: String, // default, flat, largeBiomes, amplified, customized, buffet, default_1_1
    reduced_debug: bool // 'true' causes client to hide some debug information on F3
}

impl PacketData for JoinGameData {
    fn transmit(&self, stream: &mut Stream) {
        stream.write_int(self.entity_id);
        stream.write_bytes(&[ self.game_mode ]);
        stream.write_int(self.dimension);
        stream.write_bytes(&[ self.difficulty, self.max_players ]);
        stream.write_string(self.level_type.clone());
        stream.write_bytes(&[ match self.reduced_debug { true => 1, false => 0 } ]);
    }
}


pub fn handle(stream: &mut Stream) {
    Packet::transmit(OutgoingTypes::JoinGame, stream, JoinGameData {
        entity_id: 1,
        game_mode: 1,
        dimension: 0,
        difficulty: 0,
        max_players: 0,
        level_type: String::from("default"),
        reduced_debug: false
    });

    loop {
        Packet::transmit(OutgoingTypes::KeepAlive, stream, KeepAliveData::new());
    }

}