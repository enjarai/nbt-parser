use std::io::Write;
use byteorder::{BigEndian, WriteBytesExt};
use crate::tag::{Tag, TagError};
use crate::tag::Tag::*;
use crate::tag::TagError::InvalidType;

impl Tag {
    pub fn write<T: Write>(&self, buf: &mut T) -> Result<(), TagError> {
        match self {
            CompoundTag(value) => {
                for (key, value) in value {
                    value.write_type(buf)?;
                    write_string(buf, key)?;
                    value.write_element(buf)?;
                }
                Ok(())
            },
            _ => Err(InvalidType)
        }
    }

    fn write_type<T: Write>(&self, buf: &mut T) -> Result<(), TagError> {
        buf.write_u8(self.get_type())?;
        Ok(())
    }

    fn get_type(&self) -> u8 {
        match self {
            ByteTag(_) => 0x01,
            ShortTag(_) => 0x02,
            IntTag(_) => 0x03,
            LongTag(_) => 0x04,
            FloatTag(_) => 0x05,
            DoubleTag(_) => 0x06,
            ByteArray(_) => 0x07,
            StringTag(_) => 0x08,
            ListTag(_) => 0x09,
            CompoundTag(_) => 0x0A,
            IntArray(_) => 0x0B,
            LongArray(_) => 0x0C
        }
    }

    fn write_element<T: Write>(&self, buf: &mut T) -> Result<(), TagError> {
        match self {
            ByteTag(value) => buf.write_i8(*value)?,
            ShortTag(value) => buf.write_i16::<BigEndian>(*value)?,
            IntTag(value) => buf.write_i32::<BigEndian>(*value)?,
            LongTag(value) => buf.write_i64::<BigEndian>(*value)?,
            FloatTag(value) => buf.write_f32::<BigEndian>(*value)?,
            DoubleTag(value) => buf.write_f64::<BigEndian>(*value)?,
            ByteArray(value) => {
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for byte in value {
                    buf.write_i8(*byte)?;
                }
            },
            StringTag(value) => write_string(buf, value)?,
            ListTag(value) => {
                let type_byte = value.first().map(|t| t.get_type()).unwrap_or(0x00);
                buf.write_u8(type_byte)?;
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for tag in value {
                    tag.write_element(buf)?;
                }
            },
            CompoundTag(value) => {
                for (key, value) in value {
                    value.write_type(buf)?;
                    write_string(buf, key)?;
                    value.write_element(buf)?;
                }
                buf.write_u8(0x00)?;
            },
            IntArray(value) => {
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for int in value {
                    buf.write_i32::<BigEndian>(*int)?;
                }
            },
            LongArray(value) => {
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for long in value {
                    buf.write_i64::<BigEndian>(*long)?;
                }
            }
        }
        Ok(())
    }
}

fn write_string<T: Write>(buf: &mut T, value: &str) -> Result<(), TagError> {
    let value = value.as_bytes();
    buf.write_u16::<BigEndian>(value.len() as u16)?;
    buf.write(value)?;
    Ok(())
}
