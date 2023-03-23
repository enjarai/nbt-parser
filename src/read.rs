use std::collections::HashMap;
use std::io::Read;
use std::io;
use byteorder::{BigEndian, ReadBytesExt};
use crate::tag::{Tag, TagError};

use crate::tag::Tag::*;

impl Tag {
    pub fn read<T: Read>(buf: &mut T) -> Result<Tag, TagError> {
        Tag::read_type(0x0A, buf)
    }

    fn read_type<T: Read>(data_type: u8, buf: &mut T) -> Result<Tag, TagError> {
        match data_type {
            // Byte (1 byte)
            0x01 => {
                Ok(ByteTag(buf.read_i8()?))
            },
            // Short (2 bytes)
            0x02 => {
                Ok(ShortTag(buf.read_i16::<BigEndian>()?))
            },
            // Int (4 bytes)
            0x03 => {
                Ok(IntTag(buf.read_i32::<BigEndian>()?))
            },
            // Long (8 bytes)
            0x04 => {
                Ok(LongTag(buf.read_i64::<BigEndian>()?))
            },
            // Float (4 bytes)
            0x05 => {
                Ok(FloatTag(buf.read_f32::<BigEndian>()?))
            },
            // Double (8 bytes)
            0x06 => {
                Ok(DoubleTag(buf.read_f64::<BigEndian>()?))
            },
            // Byte Array (i32 length, then length bytes)
            0x07 => {
                let length = buf.read_i32::<BigEndian>()?;
                let mut result = Vec::new();
                for _ in 0..length {
                    result.push(buf.read_i8()?)
                }
                Ok(ByteArray(result))
            },
            // String (u16 length, then length bytes)
            0x08 => {
                let length = buf.read_u16::<BigEndian>()?;
                Ok(StringTag(String::from_utf8_lossy(&eat(buf, length as usize)?).parse().unwrap()))
            },
            // List (u8 type, i32 length, then length tags of type)
            0x09 => {
                let list_type = buf.read_u8()?;
                let length = buf.read_i32::<BigEndian>()?;
                let mut result = Vec::new();
                for _ in 0..length {
                    result.push(Tag::read_type(list_type, buf)?)
                }
                Ok(ListTag(result))
            },
            // Compound (u8 type, u16 name length, name of length, then tag of type, repeat until type is 0)
            0x0A => {
                let mut result = HashMap::new();
                loop {
                    let tag_type = buf.read_u8().unwrap_or(0x00);
                    if tag_type == 0x00 {
                        break;
                    }

                    let name_length = buf.read_u16::<BigEndian>()?;
                    let name = String::from_utf8(eat(buf, name_length as usize)?)?;
                    let tag = Tag::read_type(tag_type, buf)?;
                    result.insert(name, tag);
                }
                Ok(CompoundTag(result))
            },
            0x0B => {
                let length = buf.read_i32::<BigEndian>()?;
                let mut result = Vec::new();
                for _ in 0..length {
                    result.push(buf.read_i32::<BigEndian>()?)
                }
                Ok(IntArray(result))
            },
            0x0C => {
                let length = buf.read_i32::<BigEndian>()?;
                let mut result = Vec::new();
                for _ in 0..length {
                    result.push(buf.read_i64::<BigEndian>()?)
                }
                Ok(LongArray(result))
            },
            _ => Err(TagError::InvalidType)
        }
    }
}

fn eat<T: Read>(buf: &mut T, length: usize) -> io::Result<Vec<u8>> {
    let mut result = vec![0u8; length];
    buf.read_exact(&mut result)?;
    Ok(result)
}