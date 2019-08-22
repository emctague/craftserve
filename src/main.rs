use std::thread;
use std::net::{ TcpListener, TcpStream };
use crate::api_stream::Stream;

mod api_stream;
mod packet;
mod handshake;
mod status;
mod login;
mod play;

fn main() {
    println!("[Net] Attempting to open port");
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    println!("[Net] Beginning listening");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[Net] New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| { handle_client(stream) });
            }
            Err(e) => {
                println!("[Net] Incoming connection failed: {}", e);
            }
        }
    }

    println!("[Net] Stopped accepting connections");
    drop(listener);
}

fn handle_client(stream: TcpStream) {
    let mut stream = Stream::new_tcp(stream);
    handshake::handle(&mut stream);
}