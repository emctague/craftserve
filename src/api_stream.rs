use std::net::TcpStream;
use std::io::{Read, Write};

pub enum Stream {
    TCP (TcpStream),
    Vector (Vec<u8>)
}

impl Stream {

    pub fn new_tcp(t: TcpStream) -> Stream {
        Stream::TCP(t)
    }

    pub fn new_vec() -> Stream {
        Stream::Vector(Vec::new())
    }

    /// Read a variable-length string from the stream
    pub fn read_string(&mut self) -> String {
        let length = self.read_varint();
        let mut bytes = vec![0u8; length as usize];
        self.read_bytes(&mut bytes);
        String::from_utf8(bytes).unwrap()
    }

    /// Read an unsigned short from the stream
    pub fn read_ushort(&mut self) -> u16 {
        let mut bytes = [ 0 as u8, 0 ];
        self.read_bytes(&mut bytes);
        u16::from_be_bytes(bytes)
    }

    /// Read a long value from the stream
    pub fn read_long(&mut self) -> i64 {
        let mut bytes : [u8; 8] = [0; 8];
        self.read_bytes(&mut bytes);
        i64::from_be_bytes(bytes)
    }

    /// Read a variable-length integer from the stream
    pub fn read_varint(&mut self) -> i32 {
        let mut result : i32 = 0;
        let mut num_read : i32 = 0;

        loop {
            // Read a byte from the stream
            let mut read = [ 0 as u8 ];
            self.read_bytes(&mut read);
            let read = read[0];

            // Process the value and update the resulting value
            let value = read & 0b01111111;
            result |= (value as i32) << (7 * num_read);

            // Increase the number of bytes read
            num_read += 1;
            if num_read > 5 { panic!("VarInt is too big"); }

            // Stop if we've reached the last byte
            if read & 0b10000000 == 0 { break }
        }

        result
    }

    pub fn write_int(&mut self, value: i32) {
        self.write_bytes(&value.to_be_bytes())
    }

    /// Write a variable-length integer to the stream
    pub fn write_varint(&mut self, mut value: i32) {
        loop {
            let mut byte = (value & 0b01111111) as u8;
            // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                byte |= 0b10000000;
            }

            self.write_bytes(&[ byte ]);

            if value == 0 { break }
        }
    }

    pub fn varint_len(mut value: i32) -> i32 {
        let mut len = 0;

        loop {
            let mut byte = (value & 0b01111111) as u8;
            // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                byte |= 0b10000000;
            }

            len += 1;

            if value == 0 { break }
        }

        len
    }

    /// Write a variable-length string to the stream
    pub fn write_string(&mut self, value: String) {
        self.write_varint(value.len() as i32);
        self.write_bytes(value.as_bytes());
    }

    /// Write a long value to the stream
    pub fn write_long(&mut self, value: i64) {
        self.write_bytes(&value.to_be_bytes());
    }

    /// Write bytes to the stream
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        match self {
            Stream::TCP(v) => { v.write_all(bytes).unwrap(); },
            Stream::Vector(v) => { v.write_all(bytes).unwrap(); }
        }
    }

    /// Read bytes from the stream
    pub fn read_bytes(&mut self, bytes: &mut [u8]) {
        match self {
            Stream::TCP(v) => { v.read_exact(bytes).unwrap(); },
            Stream::Vector(v) => {
                for b in bytes {
                    *b = v.remove(0)
                }
            }
        }
    }

    /// Gets the number of bytes written
    pub fn get_written(&mut self) -> i32 {
        match self {
            Stream::TCP(v) => 0,
            Stream::Vector(v) => v.len() as i32
        }
    }

    pub fn pipe_all(&mut self, target: &mut Stream) {
        let mut buf : [u8; 1] = [ 0 ];

        while self.get_written() > 0 {
            self.read_bytes(&mut buf);
            target.write_bytes(&mut buf);
        }
    }

}
