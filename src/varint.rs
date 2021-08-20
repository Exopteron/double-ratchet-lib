use std::io::Read;
use std::io::{Error, ErrorKind};
#[derive(Debug)]
pub struct VarInt {
    pub number: u32,
}
impl VarInt {
    #[allow(dead_code)]
    pub fn new(number: u32) -> VarInt {
        return VarInt { number: number };
    }
    #[allow(dead_code)]
    pub fn new_as_bytes(number: u32) -> Vec<u8> {
        let mut vint = VarInt { number: number };
        return vint.into_bytes();
    }
    pub fn new_u32_from_bytes(mut input: &mut dyn std::io::Read) -> std::io::Result<VarInt> {
        let vint = VarInt::new_from_bytes(&mut input);
        return vint;
    }
    pub fn read_string(mut input: &mut dyn std::io::Read) -> String {
        let strlen = VarInt::new_u32_from_bytes(input).unwrap().number;
        let mut string = vec![0; strlen as usize];
        input.read_exact(&mut string);
        let string = String::from_utf8_lossy(&string).to_string();
        return string;
    }
    pub fn read_varint_prefixed_bytearray(mut input: &mut dyn std::io::Read) -> Vec<u8> {
        let strlen = VarInt::new_u32_from_bytes(input).unwrap().number;
        let mut string = vec![0; strlen as usize];
        input.read_exact(&mut string);
        /*         let string = String::from_utf8_lossy(&string).to_string(); */
        return string;
    }
    pub fn read_unsigned_short(mut input: &mut dyn std::io::Read) -> usize {
        let mut string = vec![0; 2];
        input.read_exact(&mut string);
        let mut numarray = [0; 2];
        for i in 0..2 {
            numarray[i] = string[i];
        }
        let num: u16 = u16::from_be_bytes(numarray);
        let num: usize = num as usize;
        return num;
    }
    pub fn read_u128(mut input: &mut dyn std::io::Read) -> u128 {
        let mut string = vec![0; 16];
        input.read_exact(&mut string);
        let mut numarray = [0; 16];
        for i in 0..16 {
            numarray[i] = string[i];
        }
        let num: u128 = u128::from_be_bytes(numarray);
        let num: u128 = num as u128;
        return num;
    }
    pub fn read_u32(mut input: &mut dyn std::io::Read) -> u32 {
        let mut string = vec![0; 4];
        input.read_exact(&mut string);
        let mut numarray = [0; 4];
        for i in 0..4 {
            numarray[i] = string[i];
        }
        let num: u32 = u32::from_be_bytes(numarray);
        let num: u32 = num as u32;
        return num;
    }
    pub fn read_short(mut input: &mut dyn std::io::Read) -> isize {
        let mut string = vec![0; 2];
        input.read_exact(&mut string);
        let mut numarray = [0; 2];
        for i in 0..2 {
            numarray[i] = string[i];
        }
        let num: i16 = i16::from_be_bytes(numarray);
        let num: isize = num as isize;
        return num;
    }
    pub fn read_int(mut input: &mut dyn std::io::Read) -> isize {
        let mut string = vec![0; 4];
        input.read_exact(&mut string);
        let mut numarray = [0; 4];
        for i in 0..4 {
            numarray[i] = string[i];
        }
        let num: i32 = i32::from_be_bytes(numarray);
        let num: isize = num as isize;
        return num;
    }
    pub fn write_short(number: i16) -> Vec<u8> {
        let mut bytes = number.to_be_bytes().to_vec();
        if bytes.len() < 2 {
            for i in 0..2 - bytes.len() {
                bytes.reverse();
                bytes.push(0x00);
                bytes.reverse();
            }
        }
        return bytes;
    }
    pub fn write_int(number: i32) -> Vec<u8> {
        let mut bytes = number.to_be_bytes().to_vec();
        if bytes.len() < 4 {
            for i in 0..4 - bytes.len() {
                bytes.reverse();
                bytes.push(0x00);
                bytes.reverse();
            }
        }
        return bytes;
    }
    pub fn write_unsigned_short(number: u16) -> Vec<u8> {
        let mut bytes = number.to_be_bytes().to_vec();
        if bytes.len() < 2 {
            for i in 0..2 - bytes.len() {
                bytes.reverse();
                bytes.push(0x00);
                bytes.reverse();
            }
        }
        return bytes;
    }
    pub fn write_u128(number: u128) -> Vec<u8> {
        let mut bytes = number.to_be_bytes().to_vec();
        if bytes.len() < 16 {
            for i in 0..16 - bytes.len() {
                bytes.reverse();
                bytes.push(0x00);
                bytes.reverse();
            }
        }
        return bytes;
    }
    pub fn write_u32(number: u32) -> Vec<u8> {
        let mut bytes = number.to_be_bytes().to_vec();
        if bytes.len() < 4 {
            for i in 0..4 - bytes.len() {
                bytes.reverse();
                bytes.push(0x00);
                bytes.reverse();
            }
        }
        return bytes;
    }
    pub fn read_packet(mut input: &mut dyn std::io::Read) -> Result<(usize, Vec<u8>), String> {
        let length = VarInt::new_u32_from_bytes(input);
        if length.is_err() {
            return Err("Failed to read.".to_string());
        }
        let length = length.unwrap().number;
        let mut packet = vec![0; length as usize];
        input.read_exact(&mut packet);
        let mut packet = std::io::Cursor::new(packet);
        let packetid = VarInt::new_u32_from_bytes(&mut packet).unwrap().number as usize;
        let mut pack2 = vec![];
        packet.read_to_end(&mut pack2);
        return Ok((packetid, pack2));
    }
    pub fn write_pluginmessage_packet(mut input: Vec<u8>, channel: &str) -> Vec<u8> {
        let mut packet = vec![];
        packet.append(&mut VarInt::write_string(channel.to_string()));
        packet.append(&mut VarInt::write_short(input.len() as i16));
        packet.append(&mut input.clone());
        let packet = VarInt::galax_write_packet(packet, 0x17);
        return packet;
    }
    pub fn galax_write_packet(mut input: Vec<u8>, packetid: usize) -> Vec<u8> {
        let mut packetidvec = vec![];
        packetidvec.append(&mut VarInt::new_as_bytes(packetid as u32));
        let mut packet = vec![];
        packet.append(&mut VarInt::new_as_bytes(
            (packetidvec.len() + input.len()) as u32,
        ));
        packet.append(&mut packetidvec);
        packet.append(&mut input);
        return packet;
    }
    pub fn galax_write_packet_2(mut input: Vec<u8>, packetid: usize) -> Vec<u8> {
        let mut packetidvec = vec![];
        packetidvec.append(&mut VarInt::new_as_bytes(packetid as u32));
        let mut packet = vec![];
        packet.append(&mut VarInt::new_as_bytes(
            (packetidvec.len() + input.len() + 1) as u32,
        ));
        packet.append(&mut packetidvec);
        packet.append(&mut input);
        return packet;
    }
    pub fn write_packet(mut input: Vec<u8>, packetid: usize) -> Vec<u8> {
        let mut packet = vec![];
        packet.append(&mut VarInt::new_as_bytes(packetid as u32));
        packet.append(&mut input);
        packet.reverse();
        let mut fpacketlen = vec![];
        fpacketlen.append(&mut VarInt::new_as_bytes(packet.len() as u32));
        fpacketlen.reverse();
        packet.append(&mut fpacketlen);
        packet.reverse();
        return packet;
    }
    pub fn write_string(string: String) -> Vec<u8> {
        let mut packet = vec![];
        packet.append(&mut VarInt::new_as_bytes(string.as_bytes().len() as u32));
        packet.append(&mut string.as_bytes().to_vec());
        return packet;
    }
    pub fn write_varint_prefixed_bytearray(string: Vec<u8>) -> Vec<u8> {
        let mut packet = vec![];
        packet.append(&mut VarInt::new_as_bytes(string.len() as u32));
        packet.append(&mut string.to_vec());
        return packet;
    }
    pub fn into_bytes(&mut self) -> Vec<u8> {
        let mut value = self.number as u32;
        if value == 0 {
            return vec![0];
        }
        if value <= 127 && value > 0 {
            return vec![value as u8];
        }
        if value <= 255 && value > 0 {
            return vec![value as u8, 1];
        }
        let mut out = vec![];
        while value != 0 {
            let currentbyte = value & 0b01111111;
            let mut currentbyte = currentbyte as u8;
            value >>= 7;
            if value != 0 {
                currentbyte |= 0b10000000;
            }
            out.push(currentbyte);
        }
        return out;
    }
    pub fn new_from_bytes(inputreader: &mut dyn std::io::Read) -> std::io::Result<VarInt> {
        let mut value: i32 = 0;
        let mut bitoffset = 0;
        let mut currentbyte = 0;
        let mut set = true;
        while (currentbyte & 0b10000000) != 0 || set == true {
            if bitoffset == 35 {
                return Err(Error::new(ErrorKind::Other, "VarInt too large!"));
            }
            currentbyte = Self::read_byte(inputreader)? as i32;
            value |= (currentbyte & 0b01111111) << bitoffset;
            bitoffset += 7;
            set = false;
        }
        return Ok(VarInt { number: value as u32 });
    }
    fn read_byte(stream: &mut dyn std::io::Read) -> std::io::Result<u8> {
        let mut x = [0; 1];
        stream.read_exact(&mut x)?;
        return Ok(x[0]);
    }
}
