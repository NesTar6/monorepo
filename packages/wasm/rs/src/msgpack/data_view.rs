use super::{Context, BLOCK_MAX_SIZE, E_INVALID_LENGTH};
use std::io::Cursor;

#[derive(Clone, Debug, Default)]
pub struct DataView {
    pub(crate) buffer: Cursor<Vec<u8>>,
    pub(crate) context: Context,
}

impl DataView {
    pub fn new(buf: &[u8], context: Context) -> Result<Self, String> {
        // let byte_length = buf.len();
        // if byte_length > BLOCK_MAX_SIZE {
        //     return Err(context.print_with_context(&format!(
        //         "DataView::new(): {} [ byte_length: {} ]",
        //         E_INVALID_LENGTH, byte_length
        //     )));
        // }
        Ok(Self {
            buffer: Cursor::new(buf.to_vec()),
            context,
        })
    }

    // pub fn get_bytes(&mut self, length: usize) -> Vec<u8> {
    //     self.check_index_in_range("get_bytes", length);
    //     let buf = self.buffer.as_slice();
    //     let (b_off, b_len) = (self.byte_offset, self.byte_offset + length);
    //     let result = &buf[b_off..b_len];
    //     self.byte_offset += length;
    //     result.to_vec()
    // }

    // pub fn peek_u8(&mut self) -> u8 {
    //     self.check_index_in_range("peek_u8", 0);
    //     let u8_ptr = (self.data_start + self.byte_offset) as *const u8;
    //     u8::swap_bytes(u8_ptr as u8)
    // }

    // pub fn discard(&mut self, length: usize) {
    //     self.check_index_in_range("discard", length);
    //     self.byte_offset += length;
    // }

    // pub fn get_f32(&mut self) -> f32 {
    //     self.check_index_in_range("get_f32", 4);
    //     let f32_ptr = (self.data_start + self.byte_offset) as *const f32;
    //     self.byte_offset += 4;
    //     u32::swap_bytes(f32_ptr as u32) as f32
    // }

    // pub fn get_f64(&mut self) -> f64 {
    //     self.check_index_in_range("get_f64", 8);
    //     let f64_ptr = (self.data_start + self.byte_offset) as *const f64;
    //     self.byte_offset += 8;
    //     u64::swap_bytes(f64_ptr as u64) as f64
    // }

    // pub fn get_i8(&mut self) -> i8 {
    //     self.check_index_in_range("get_i8", 1);
    //     let i8_ptr = (self.data_start + self.byte_offset) as *const i8;
    //     self.byte_offset += 1;
    //     i8::swap_bytes(i8_ptr as i8)
    // }

    // pub fn get_i16(&mut self) -> i16 {
    //     self.check_index_in_range("get_i16", 2);
    //     let i16_ptr = (self.data_start + self.byte_offset) as *const i16;
    //     self.byte_offset += 2;
    //     i16::swap_bytes(i16_ptr as i16)
    // }

    // pub fn get_i32(&mut self) -> i32 {
    //     self.check_index_in_range("get_i32", 4);
    //     let i32_ptr = (self.data_start + self.byte_offset) as *const i32;
    //     self.byte_offset += 4;
    //     i32::swap_bytes(i32_ptr as i32)
    // }

    // pub fn get_i64(&mut self) -> i64 {
    //     self.check_index_in_range("get_i64", 8);
    //     let i64_ptr = (self.data_start + self.byte_offset) as *const i64;
    //     self.byte_offset += 8;
    //     i64::swap_bytes(i64_ptr as i64)
    // }

    // pub fn get_u8(&mut self) -> u8 {
    //     self.check_index_in_range("get_u8", 1);
    //     let u8_ptr = (self.data_start + self.byte_offset) as *const u8;
    //     self.byte_offset += 1;
    //     u8::swap_bytes(u8_ptr as u8)
    // }

    // pub fn get_u16(&mut self) -> u16 {
    //     self.check_index_in_range("get_u16", 2);
    //     let u16_ptr = (self.data_start + self.byte_offset) as *const u16;
    //     self.byte_offset += 2;
    //     u16::swap_bytes(u16_ptr as u16)
    // }

    // pub fn get_u32(&mut self) -> u32 {
    //     self.check_index_in_range("get_u32", 4);
    //     let u32_ptr = (self.data_start + self.byte_offset) as *const u32;
    //     self.byte_offset += 4;
    //     u32::swap_bytes(u32_ptr as u32)
    // }

    // pub fn get_u64(&mut self) -> u64 {
    //     self.check_index_in_range("get_u64", 8);
    //     let u64_ptr = (self.data_start + self.byte_offset) as *const u64;
    //     self.byte_offset += 8;
    //     u64::swap_bytes(u64_ptr as u64)
    // }

    // pub fn set_bytes(&mut self, buf: &[u8]) {
    //     self.check_index_in_range("set_bytes", buf.len());
    //     self.extend_from_slice(buf);
    //     self.byte_offset += buf.len();
    // }

    // pub fn set_f32(&mut self, value: f32) {
    //     self.check_index_in_range("set_f32", 4);
    //     let swapped_f32 = (value as u32).swap_bytes() as f32;
    //     self.extend_from_slice(&swapped_f32.to_be_bytes());
    //     self.byte_offset += 4;
    // }

    // pub fn set_f64(&mut self, value: f64) {
    //     self.check_index_in_range("set_f64", 8);
    //     let swapped_f64 = (value as u64).swap_bytes() as f64;
    //     self.extend_from_slice(&swapped_f64.to_be_bytes());
    //     self.byte_offset += 8;
    // }

    // pub fn set_i8(&mut self, value: i8) {
    //     self.check_index_in_range("set_i8", 1);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 1;
    // }

    // pub fn set_i16(&mut self, value: i16) {
    //     self.check_index_in_range("set_i16", 2);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 2;
    // }

    // pub fn set_i32(&mut self, value: i32) {
    //     self.check_index_in_range("set_i32", 4);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 4;
    // }

    // pub fn set_i64(&mut self, value: i64) {
    //     self.check_index_in_range("set_i64", 8);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 8;
    // }

    // pub fn set_u8(&mut self, value: u8) {
    //     self.check_index_in_range("set_u8", 1);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 1;
    // }

    // pub fn set_u16(&mut self, value: u16) {
    //     self.check_index_in_range("set_u16", 2);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 2;
    // }

    // pub fn set_u32(&mut self, value: u32) {
    //     self.check_index_in_range("set_u32", 4);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 4;
    // }

    // pub fn set_u64(&mut self, value: u64) {
    //     self.check_index_in_range("set_u64", 8);
    //     self.extend_from_slice(&value.swap_bytes().to_be_bytes());
    //     self.byte_offset += 8;
    // }

    // pub fn get_byte_length(&self) -> i32 {
    //     self.byte_length as i32
    // }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.clone().into_inner()
    }

    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    // fn check_index_in_range(&self, method: &str, length: usize) {
    //     let byte_length = self.get_buffer().len();
    //     if length > byte_length {
    //         super::utils::throw_index_out_of_range(&self.context, method, length, byte_length);
    //     }
    // }

    // #[inline]
    // fn extend_from_slice(&mut self, buf: &[u8]) {
    //     self.buffer.clear();
    //     self.buffer.extend_from_slice(buf);
    // }
}
