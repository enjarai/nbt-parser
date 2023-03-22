use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, ErrorKind, Read};
use std::string::{FromUtf8Error, ParseError};
use std::{io, u16};
use std::fmt::{Debug, Display, Formatter};
use std::mem::transmute;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use crate::parser::Tag::{ByteTag, ByteArray, DoubleTag, EndTag, FloatTag, IntTag, LongTag, ShortTag, StringTag, ListTag, CompoundTag, IntArray};

// struct Parser<'a> {
//     data: &'a Vec<u8>,
//     i: usize
// }
//
// impl<'a> Parser<'a> {
//     fn new(data: &Vec<u8>) -> Parser {
//         Parser {
//             data,
//             i: 0
//         }
//     }
//
//     fn eat(&mut self, count: usize) -> &Vec<u8> {
//         let output = self.data[i..i+count];
//         self.i += count;
//         output
//     }
// }

#[derive(Debug)]
pub enum TagParseErr {
    InvalidType,
    InvalidLength,
    InvalidUtf8,
    IoError(io::Error)
}

impl From<io::Error> for TagParseErr {
    fn from(err: io::Error) -> TagParseErr {
        TagParseErr::IoError(err)
    }
}

impl From<FromUtf8Error> for TagParseErr {
    fn from(_: FromUtf8Error) -> TagParseErr {
        TagParseErr::InvalidUtf8
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
            0x01 => unsafe {
                Ok(ByteTag(buf.read_i8()?))
            },
            0x02 => {
                Ok(ShortTag(buf.read_i16::<BigEndian>()?))
            },
            0x03 => {
                Ok(IntTag(buf.read_i32::<BigEndian>()?))
            },
            0x04 => {
                Ok(LongTag(buf.read_i64::<BigEndian>()?))
            },
            0x05 => {
                Ok(FloatTag(buf.read_f32::<BigEndian>()?))
            },
            0x06 => {
                Ok(DoubleTag(buf.read_f64::<BigEndian>()?))
            },
            // 0x07 => {
            //     let length = i32::from_be_bytes(*eat(&mut data, 4)?);
            //     let mut result = Vec::new();
            //     for b in data[0..length] {
            //         result.push(i8::from_be_bytes(b))
            //     }
            //     Ok(ByteArray(result))
            // },
            0x08 => {
                let length = buf.read_u16::<BigEndian>()?;
                Ok(StringTag(String::from_utf8(eat(buf, length as usize)?)?))
            },
            0x09 => {
                let list_type = buf.read_u8()?;
                let length = buf.read_i32::<BigEndian>()?;
                let mut result = Vec::new();
                for _ in 0..length {
                    result.push(Tag::parse_type(list_type, buf)?)
                }
                Ok(ListTag(result))
            },
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
            // 0x0B => {
            //     let length = i32::from_be_bytes(*eat(&mut data, 4)?);
            //     let mut result = Vec::new();
            //     for b in data[0..length] {
            //         result.push(i32::from_be_bytes(b))
            //     }
            //     Ok(IntArray(result))
            // },
            _ => Err(TagParseErr::InvalidType)
        }
    }
}

fn eat(buf: &mut BufReader<impl Read>, length: usize) -> io::Result<Vec<u8>> {
    let mut result = vec![0u8; length];
    buf.read_exact(&mut result)?;
    Ok(result)
}