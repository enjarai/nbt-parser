use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use std::io;
use crate::tag::Tag::*;

#[derive(Debug)]
pub enum TagError {
    InvalidType,
    InvalidUtf8(FromUtf8Error),
    IoError(io::Error)
}

impl From<io::Error> for TagError {
    fn from(err: io::Error) -> TagError {
        TagError::IoError(err)
    }
}

impl From<FromUtf8Error> for TagError {
    fn from(err: FromUtf8Error) -> TagError {
        TagError::InvalidUtf8(err)
    }
}

#[derive(Debug)]
pub enum Tag {
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

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
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

impl Tag {
    pub fn count_elements(&self, counter: &mut usize) {
        match self {
            ByteTag(_) => *counter += 1,
            ShortTag(_) => *counter += 1,
            IntTag(_) => *counter += 1,
            LongTag(_) => *counter += 1,
            FloatTag(_) => *counter += 1,
            DoubleTag(_) => *counter += 1,
            ByteArray(value) => *counter += value.len(),
            StringTag(_) => *counter += 1,
            ListTag(value) => {
                for tag in value {
                    tag.count_elements(counter);
                }
            }
            CompoundTag(value) => {
                for tag in value.values() {
                    tag.count_elements(counter);
                }
            }
            IntArray(value) => *counter += value.len(),
            LongArray(value) => *counter += value.len()
        }
    }

    pub fn byte(&self) -> Option<&i8> {
        match self {
            ByteTag(b) => Some(b),
            _ => None
        }
    }

    pub fn short(&self) -> Option<&i16> {
        match self {
            ShortTag(s) => Some(s),
            _ => None
        }
    }

    pub fn int(&self) -> Option<&i32> {
        match self {
            IntTag(i) => Some(i),
            _ => None
        }
    }

    pub fn long(&self) -> Option<&i64> {
        match self {
            LongTag(l) => Some(l),
            _ => None
        }
    }

    pub fn float(&self) -> Option<&f32> {
        match self {
            FloatTag(f) => Some(f),
            _ => None
        }
    }

    pub fn double(&self) -> Option<&f64> {
        match self {
            DoubleTag(d) => Some(d),
            _ => None
        }
    }

    pub fn byte_array(&self) -> Option<&Vec<i8>> {
        match self {
            ByteArray(b) => Some(b),
            _ => None
        }
    }

    pub fn string(&self) -> Option<&String> {
        match self {
            StringTag(s) => Some(s),
            _ => None
        }
    }

    pub fn list(&self) -> Option<&Vec<Tag>> {
        match self {
            ListTag(l) => Some(l),
            _ => None
        }
    }

    pub fn compound(&self) -> Option<&HashMap<String, Tag>> {
        match self {
            CompoundTag(c) => Some(c),
            _ => None
        }
    }

    pub fn int_array(&self) -> Option<&Vec<i32>> {
        match self {
            IntArray(i) => Some(i),
            _ => None
        }
    }

    pub fn long_array(&self) -> Option<&Vec<i64>> {
        match self {
            LongArray(l) => Some(l),
            _ => None
        }
    }

    pub fn byte_mut(&mut self) -> Option<&mut i8> {
        match self {
            ByteTag(b) => Some(b),
            _ => None
        }
    }

    pub fn short_mut(&mut self) -> Option<&mut i16> {
        match self {
            ShortTag(s) => Some(s),
            _ => None
        }
    }

    pub fn int_mut(&mut self) -> Option<&mut i32> {
        match self {
            IntTag(i) => Some(i),
            _ => None
        }
    }

    pub fn long_mut(&mut self) -> Option<&mut i64> {
        match self {
            LongTag(l) => Some(l),
            _ => None
        }
    }

    pub fn float_mut(&mut self) -> Option<&mut f32> {
        match self {
            FloatTag(f) => Some(f),
            _ => None
        }
    }

    pub fn double_mut(&mut self) -> Option<&mut f64> {
        match self {
            DoubleTag(d) => Some(d),
            _ => None
        }
    }

    pub fn byte_array_mut(&mut self) -> Option<&mut Vec<i8>> {
        match self {
            ByteArray(b) => Some(b),
            _ => None
        }
    }

    pub fn string_mut(&mut self) -> Option<&mut String> {
        match self {
            StringTag(s) => Some(s),
            _ => None
        }
    }

    pub fn list_mut(&mut self) -> Option<&mut Vec<Tag>> {
        match self {
            ListTag(l) => Some(l),
            _ => None
        }
    }

    pub fn compound_mut(&mut self) -> Option<&mut HashMap<String, Tag>> {
        match self {
            CompoundTag(c) => Some(c),
            _ => None
        }
    }

    pub fn int_array_mut(&mut self) -> Option<&mut Vec<i32>> {
        match self {
            IntArray(i) => Some(i),
            _ => None
        }
    }

    pub fn long_array_mut(&mut self) -> Option<&mut Vec<i64>> {
        match self {
            LongArray(l) => Some(l),
            _ => None
        }
    }
}
