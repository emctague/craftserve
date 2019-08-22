use num_traits::{ FromPrimitive };
use num_derive::{ FromPrimitive, ToPrimitive };
use crate::api_stream::Stream;
use crate::packet::Packet;
use crate::{status, login};

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum IncomingTypes {
    Handshake = 0x00
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum NextState {
    Status = 1,      // Client is seeking server status information
    Login = 2       // Client is seeking to log in
}

#[derive(Debug)]
struct HandshakeData {
    pub version: i32,    // Protocol Version VarInt
    pub address: String, // Hostname / IP used to connect to server
    pub port: u16,       // Port used to connect to server
    pub next_state: NextState // Intended state after this server state
}

impl HandshakeData {
    pub fn new(stream: &mut Stream) -> HandshakeData {
        let version = stream.read_varint();
        let address = stream.read_string();
        let port = stream.read_ushort();
        let next_state = stream.read_varint();
        let next_state = FromPrimitive::from_i32(next_state).unwrap();
        HandshakeData { version, address, port, next_state }
    }
}

pub fn handle(stream: &mut Stream) {
    println!("[Handshake] Waiting for handshake");

    let packet : Packet<IncomingTypes> = Packet::new(stream);
    let data = HandshakeData::new(stream);
    println!("[Handshake] Handshake received: {:#?}", data);

    println!("[Handshake] Changing to state: {:#?}", data.next_state);

    match data.next_state {
        NextState::Status => { status::handle(stream) },
        NextState::Login => { login::handle(stream) }
    }
}