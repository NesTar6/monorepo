pub mod serialization;
use crate::CustomType;
use crate::{Read, Write};
use serde::{Deserialize, Serialize};
pub use serialization::{
    deserialize_another_type, read_another_type, serialize_another_type, write_another_type,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnotherType {
    pub prop: Option<String>,
    pub circular: Box<Option<CustomType>>,
}

impl AnotherType {
    pub fn new() -> AnotherType {
        AnotherType {
            prop: None,
            circular: Box::new(None),
        }
    }

    pub fn to_buffer(object: &AnotherType) -> Vec<u8> {
        serialize_another_type(object)
    }

    pub fn from_buffer(buffer: &[u8]) -> AnotherType {
        deserialize_another_type(buffer)
    }

    pub fn write<W: Write>(object: &AnotherType, writer: &mut W) {
        write_another_type(object, writer);
    }

    pub fn read<R: Read>(reader: &mut R) -> AnotherType {
        read_another_type(reader)
    }
}