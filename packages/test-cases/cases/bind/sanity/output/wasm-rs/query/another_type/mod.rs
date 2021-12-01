use serde::{Serialize, Deserialize};
pub mod serialization;
use polywrap_wasm_rs::{
    BigInt,
    Read,
    Write,
    JSON
};
pub use serialization::{
    deserialize_another_type,
    read_another_type,
    serialize_another_type,
    write_another_type
};

use crate::CustomType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnotherType {
    pub prop: Option<String>,
    pub circular: Option<CustomType>,
}

impl AnotherType {
    pub fn new() -> AnotherType {
        AnotherType {
            prop: None,
            circular: None,
        }
    }

    pub fn to_buffer(input: &AnotherType) -> Vec<u8> {
        serialize_another_type(input)
    }

    pub fn from_buffer(input: &[u8]) -> AnotherType {
        deserialize_another_type(input)
    }

    pub fn write<W: Write>(input: &AnotherType, writer: &mut W) {
        write_another_type(input, writer);
    }

    pub fn read<R: Read>(reader: &mut R) -> AnotherType {
        read_another_type(reader).expect("Failed to read AnotherType")
    }
}
