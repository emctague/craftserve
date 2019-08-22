use num_traits::FromPrimitive;
use num_derive::{ FromPrimitive, ToPrimitive };
use crate::api_stream::Stream;
use crate::packet::{Packet, PacketData};
use crate::login::IncomingTypes::LoginStart;
use uuid::Uuid;
use crate::play;

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum IncomingTypes {
    LoginStart = 0x00
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum OutgoingTypes {
    Disconnect = 0x00,
    LoginSuccess = 0x02
}

#[derive(Debug)]
struct LoginStartData {
    username: String
}

#[derive(Debug)]
struct LoginSuccessData {
    uuid: String,
    username: String
}

impl PacketData for LoginSuccessData {
    fn transmit(&self, stream: &mut Stream) {
        stream.write_string(self.uuid.clone());
        stream.write_string(self.username.clone());
    }
}

pub fn handle(stream: &mut Stream) {
    println!("[Status] Waiting for login start");

    // Wait for request packet, which is empty
    let packet : Packet<IncomingTypes> = Packet::new(stream);
    let start_data = LoginStartData { username: stream.read_string() };
    println!("[Status] Got login start: {:#?}", start_data);
    println!("[Status] Sending login success");

    Packet::transmit(OutgoingTypes::LoginSuccess, stream, LoginSuccessData {
        uuid: Uuid::new_v3(&Uuid::nil(), format!("OfflinePlayer:<{}>", start_data.username).as_bytes()).to_hyphenated().to_string(),
        username: start_data.username
    });

    println!("[Status] Logged in");

    play::handle(stream);
}