use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::string::FromUtf8Error;
use std::io;
use std::fmt::{Debug, Display, Formatter};
use byteorder::{BigEndian, ReadBytesExt};

use crate::parser::Tag::{ByteTag, ByteArray, DoubleTag, EndTag, FloatTag, IntTag, LongTag, ShortTag, StringTag, ListTag, CompoundTag, IntArray, LongArray};

#[derive(Debug)]
pub enum TagParseErr {
    InvalidType,
    InvalidUtf8(FromUtf8Error),
    IoError(io::Error)
}

impl From<io::Error> for TagParseErr {
    fn from(err: io::Error) -> TagParseErr {
        TagParseErr::IoError(err)
    }
}

impl From<FromUtf8Error> for TagParseErr {
    fn from(err: FromUtf8Error) -> TagParseErr {
        TagParseErr::InvalidUtf8(err)
    }
}

#[derive(Debug)]
pub enum Tag {
    EndTag,
    ByteTag(i8),
    ShortTag(i16),
    IntTag(i32),
    LongTag(i64),
    FloatTag(f32),
    DoubleTag(f64),
    ByteArray(Vec<i8>),
    StringTag(String),
    ListTag(Vec<Tag>),
    CompoundTag(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

impl Tag {
    pub fn parse(buf: &mut BufReader<impl Read>) -> Result<Tag, TagParseErr> {
        Tag::parse_type(0x0A, buf)
    }

    fn parse_type(data_type: u8, buf: &mut BufReader<impl Read>) -> Result<Tag, TagParseErr> {
        match data_type {
            0x00 => Ok(EndTag),
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
                    result.push(Tag::parse_type(list_type, buf)?)
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
                    let tag = Tag::parse_type(tag_type, buf)?;
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
            _ => Err(TagParseErr::InvalidType)
        }
    }
}

fn eat(buf: &mut BufReader<impl Read>, length: usize) -> io::Result<Vec<u8>> {
    let mut result = vec![0u8; length];
    buf.read_exact(&mut result)?;
    Ok(result)
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EndTag => write!(f, "EndTag"),
            ByteTag(value) => Display::fmt(value, f),
            ShortTag(value) => Display::fmt(value, f),
            IntTag(value) => Display::fmt(value, f),
            LongTag(value) => Display::fmt(value, f),
            FloatTag(value) => Display::fmt(value, f),
            DoubleTag(value) => Display::fmt(value, f),
            ByteArray(value) => list_fmt(value, f),
            StringTag(value) => Display::fmt(value, f),
            ListTag(value) => list_fmt(value, f),
            CompoundTag(value) => {
                write!(f, "{{")?;
                for (i, (key, tag)) in value.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, tag)?;
                }
                write!(f, "}}")
            }
            IntArray(value) => list_fmt(value, f),
            LongArray(value) => list_fmt(value, f)
        }
    }
}

fn list_fmt<T: Display>(value: &Vec<T>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for (i, t) in value.iter().enumerate() {
        if i != 0 {
            write!(f, ", ")?;
        }
        write!(f, "{}", t)?;
    }
    write!(f, "]")
}