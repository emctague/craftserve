use num_traits::FromPrimitive;
use num_derive::{ FromPrimitive, ToPrimitive };
use crate::api_stream::Stream;
use crate::packet::{Packet, PacketData};
use crate::status::GameVersion::V1_14_4;

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum IncomingTypes {
    Request = 0x00,
    Ping = 0x01
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum OutgoingTypes {
    Response = 0x00,
    Pong = 0x01
}

#[derive(Debug)]
enum GameVersion {
    V1_14_4
}

impl GameVersion {

    pub fn name(&self) -> String {
        match self {
            V1_14_4 => "1.14.4"
        }.to_string()
    }

    pub fn id(&self) -> i32 {
        match self {
            V1_14_4 => 498
        }
    }

}

#[derive(Debug)]
struct ResponseData {
    version: GameVersion,
    max_players: i32,
    online_players: i32,
    sample: String,
    description: String,
    favicon: String
}

impl PacketData for ResponseData {
    fn transmit(&self, stream: &mut Stream) {
        stream.write_string(format!("{{\
        \"version\": {{ \"name\": \"{}\", \"protocol\": {} }},
        \"players\": {{ \"max\": {}, \"online\": {}, \"sample\": [ {} ] }},
        \"description\": {},
        \"favicon\": \"{}\"
        }}",
            self.version.name(), self.version.id(),
            self.max_players, self.online_players, self.sample,
            self.description,
            self.favicon));
    }
}

#[derive(Debug)]
struct PongData(i64);

impl PacketData for PongData {
    fn transmit(&self, stream: &mut Stream) {
        stream.write_long(self.0);
    }
}


pub fn handle(stream: &mut Stream) {
    println!("[Status] Waiting for request");

    // Wait for request packet, which is empty
    let packet : Packet<IncomingTypes> = Packet::new(stream);
    println!("[Status] Got request: {:#?}", packet);

    Packet::transmit(OutgoingTypes::Response, stream, ResponseData {
        version: GameVersion::V1_14_4,
        max_players: 420,
        online_players: 69,
        sample: "{ \"name\": \"Shrek\", \"id\": \"4566e69f-c907-48ee-8d71-d7ba5aa00d20\" }".to_string(),
        description: String::from(r#"
            {
                "text": "GET OUT OF ME ",
                "bold": true,
                "italic": true,
                "color": "aqua",
                "extra": [
                    {
                        "text": "SWAMP!",
                        "bold": true,
                        "italic": true,
                        "underlined": true,
                        "color": "red",
                        "clickEvent": { "action": "open_url", "value": "https://www.youtube.com/watch?v=QhTf_f0bvkQ" }
                    }
                ]
            }
        "#),
        favicon: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAACXBIWXMAAAsTAAALEwEAmpwYAAAFHGlUWHRYTUw6Y29tLmFkb2JlLnhtcAAAAAAAPD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0wTXBDZWhpSHpyZVN6TlRjemtjOWQiPz4gPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyIgeDp4bXB0az0iQWRvYmUgWE1QIENvcmUgNS42LWMxNDUgNzkuMTYzNDk5LCAyMDE4LzA4LzEzLTE2OjQwOjIyICAgICAgICAiPiA8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPiA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0iIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtbG5zOmRjPSJodHRwOi8vcHVybC5vcmcvZGMvZWxlbWVudHMvMS4xLyIgeG1sbnM6cGhvdG9zaG9wPSJodHRwOi8vbnMuYWRvYmUuY29tL3Bob3Rvc2hvcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RFdnQ9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZUV2ZW50IyIgeG1wOkNyZWF0b3JUb29sPSJBZG9iZSBQaG90b3Nob3AgQ0MgMjAxOSAoV2luZG93cykiIHhtcDpDcmVhdGVEYXRlPSIyMDE5LTA4LTE5VDIyOjIzOjE2LTA0OjAwIiB4bXA6TW9kaWZ5RGF0ZT0iMjAxOS0wOC0xOVQyMjoyNjo1Ni0wNDowMCIgeG1wOk1ldGFkYXRhRGF0ZT0iMjAxOS0wOC0xOVQyMjoyNjo1Ni0wNDowMCIgZGM6Zm9ybWF0PSJpbWFnZS9wbmciIHBob3Rvc2hvcDpDb2xvck1vZGU9IjMiIHBob3Rvc2hvcDpJQ0NQcm9maWxlPSJzUkdCIElFQzYxOTY2LTIuMSIgeG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDo0NDhmNzRkNS1iMDgx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".to_string()
    });
    println!("[Status] Sent response");

    println!("[Status] Waiting for ping");

    // Wait for request packet, which is empty
    let packet : Packet<IncomingTypes> = Packet::new(stream);
    let ping = stream.read_long();
    println!("[Status] Got ping: {:#?}", ping);

    Packet::transmit(OutgoingTypes::Pong, stream, PongData(ping));
    println!("[Status] Sent pong");
}