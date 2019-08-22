use crate::api_stream::Stream;
use std::fmt::Debug;
use num_traits::{ FromPrimitive, ToPrimitive };

pub trait PacketData {
    fn transmit(&self, stream: &mut Stream);
}

pub struct NoData;

impl PacketData for NoData {
    fn transmit(&self, stream: &mut Stream) {
    }
}

#[derive(Debug)]
pub struct Packet<T: FromPrimitive + ToPrimitive + Debug> {
    pub length: i32,
    pub id: T
}

impl<T: FromPrimitive + ToPrimitive + Debug> Packet<T> {
    pub fn new(stream: &mut Stream) -> Packet<T> {
        let length = stream.read_varint();
        let id = stream.read_varint();
        let id = FromPrimitive::from_i32(id).unwrap();
        Packet { length, id }
    }

    pub fn send(&self, stream: &mut Stream) {
        stream.write_varint(self.length);
        stream.write_varint(self.id.to_i32().unwrap());
    }

    pub fn transmit<Y: PacketData> (id: T, stream: &mut Stream, data: Y) {
        let data_stream = &mut Stream::new_vec();
        data.transmit(data_stream);
        let id = id.to_i32().unwrap();

        let packet = Packet {
            length: data_stream.get_written() + Stream::varint_len(id),
            id
        };

        packet.send(stream);

        data_stream.pipe_all(stream);
    }
}