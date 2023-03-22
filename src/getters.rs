use std::collections::HashMap;
use crate::parser::Tag;

impl Tag {
    pub fn get_as_byte(&self) -> Option<i8> {
        match self {
            Tag::ByteTag(b) => Some(*b),
            _ => None
        }
    }

    pub fn get_as_short(&self) -> Option<i16> {
        match self {
            Tag::ShortTag(s) => Some(*s),
            _ => None
        }
    }

    pub fn get_as_int(&self) -> Option<i32> {
        match self {
            Tag::IntTag(i) => Some(*i),
            _ => None
        }
    }

    pub fn get_as_long(&self) -> Option<i64> {
        match self {
            Tag::LongTag(l) => Some(*l),
            _ => None
        }
    }

    pub fn get_as_float(&self) -> Option<f32> {
        match self {
            Tag::FloatTag(f) => Some(*f),
            _ => None
        }
    }

    pub fn get_as_double(&self) -> Option<f64> {
        match self {
            Tag::DoubleTag(d) => Some(*d),
            _ => None
        }
    }

    pub fn get_as_byte_array(&self) -> Option<&Vec<i8>> {
        match self {
            Tag::ByteArray(b) => Some(b),
            _ => None
        }
    }

    pub fn get_as_string(&self) -> Option<&String> {
        match self {
            Tag::StringTag(s) => Some(s),
            _ => None
        }
    }

    pub fn get_as_list(&self) -> Option<&Vec<Tag>> {
        match self {
            Tag::ListTag(l) => Some(l),
            _ => None
        }
    }

    pub fn get_as_compound(&self) -> Option<&HashMap<String, Tag>> {
        match self {
            Tag::CompoundTag(c) => Some(c),
            _ => None
        }
    }

    pub fn get_as_int_array(&self) -> Option<&Vec<i32>> {
        match self {
            Tag::IntArray(i) => Some(i),
            _ => None
        }
    }

    pub fn get_as_long_array(&self) -> Option<&Vec<i64>> {
        match self {
            Tag::LongArray(l) => Some(l),
            _ => None
        }
    }
}