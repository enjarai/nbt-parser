use std::collections::HashMap;
use std::error::Error;
use std::string::ParseError;
use std::u16;
use crate::parser::Tag::{ByteTag, ByteArray, DoubleTag, EndTag, FloatTag, IntTag, LongTag, ShortTag, StringTag};

struct Parser<'a> {
    data: &'a Vec<u8>,
    i: usize
}

impl<'a> Parser<'a> {
    fn new(data: &Vec<u8>) -> Parser {
        Parser {
            data,
            i: 0
        }
    }

    fn eat(&mut self, count: usize) -> &Vec<u8> {
        let output = self.data[i..i+count];
        self.i += count;
        output
    }
}

enum Tag {
    EndTag,
    ByteTag(i8),
    ShortTag(i16),
    IntTag(i32),
    LongTag(i64),
    FloatTag(f32),
    DoubleTag(f64),
    ByteArray(Vec<i8>),
    StringTag(StringTag),
    ListTag(Vec<Tag>),
    CompoundTag(HashMap<StringTag, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

impl Tag {
    fn parse(data: &Vec<u8>) -> Result<Tag, ParseError> {
        let data_type = data[0];
        let mut data = data[1..data.len()];
        Tag::parse_type(data_type, &data)
    }

    fn parse_type(data_type: u8, data: &[u8]) -> Result<Tag, ParseError> {
        let mut data = *data;
        match data_type {
            0x0 => Ok(EndTag),
            0x1 => {
                if data.len() < 1 {
                    Err(ParseError)
                }
                Ok(ByteTag(i8::from_be_bytes(*data)))
            },
            0x2 => {
                if data.len() < 2 {
                    Err(ParseError)
                }
                Ok(ShortTag(i16::from_be_bytes(*data)))
            },
            0x3 => {
                if data.len() < 4 {
                    Err(ParseError)
                }
                Ok(IntTag(i32::from_be_bytes(*data)))
            },
            0x4 => {
                if data.len() < 8 {
                    Err(ParseError)
                }
                Ok(LongTag(i64::from_be_bytes(*data)))
            },
            0x5 => {
                if data.len() < 4 {
                    Err(ParseError)
                }
                Ok(FloatTag(f32::from_be_bytes(*data)))
            },
            0x6 => {
                if data.len() < 8 {
                    Err(ParseError)
                }
                Ok(DoubleTag(f64::from_be_bytes(*data)))
            },
            0x7 => {
                let length = i32::from_be_bytes(*eat(&mut data, 4)?);
                let mut result = Vec::new();
                for b in data[0..length] {
                    result.push(i8::from_be_bytes(b))
                }
                Ok(ByteArray(result))
            },
            0x8 => {
                let length = u16::from_be_bytes(*eat(&mut data, 2)?);
                Ok(StringTag(String::from_utf8(data[0..length])))
            },
            0x9 => {

            }
        }
    }
}

fn eat(data: &mut [u8], length: usize) -> Result<[u8], ParseError> {
    let result = data[0..length];
    *data = data[length..data.len()];
    Ok(result)
}