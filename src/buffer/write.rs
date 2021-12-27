use std::any::Any;
use crate::SonetReadBuf;

/// Writable Buf
pub struct SonetWriteBuf {

    /// Data of written buffers
    data: Vec<u8>,

    /// Current position
    position: i32
}

/// Writable Buf Implementation
impl SonetWriteBuf {

    /// New SonetWriteBuf
    pub fn new() -> Self {
        Self { data: vec![], position: 0 }
    }

    /// Convert to SonetReadBuf
    pub fn readable(&mut self) -> SonetReadBuf {
        SonetReadBuf::new(self.data.to_vec())
    }

    /// Write byte or u8
    pub fn write_byte(&mut self, data: u8) {
        self.write_raw(data.to_be_bytes().as_slice());
    }

    /// Write short or u16
    pub fn write_short(&mut self, data: u16) {
        self.write_raw(data.to_be_bytes().as_slice());
    }

    /// Write int or u32
    pub fn write_int(&mut self, data: u32) {
        self.write_raw(data.to_be_bytes().as_slice());
    }

    /// Write long or u64
    pub fn write_long(&mut self, data: u64) {
        self.write_raw(data.to_be_bytes().as_slice());
    }

    /// Write 0 if false, 1 if true
    pub fn write_bool(&mut self, data: bool) {
        if data {
            self.write_byte(1_u8.to_owned());
        } else {
            self.write_byte(0_u8.to_owned());
        }
    }

    /// Write byte_array or vec<u8>
    pub fn write_byte_array(&mut self, data: &Vec<u8>) {
        self.write_int(data.len() as u32);
        for datum in data.to_vec().into_iter() {
            self.write_byte(datum);
        }
    }

    /// Write short_array or vec<u16>
    pub fn write_short_array(&mut self, data: &Vec<u16>) {
        self.write_int(data.len() as u32);
        for datum in data.to_vec().into_iter() {
            self.write_short(datum);
        }
    }

    /// Write int_array or vec<u32>
    pub fn write_int_array(&mut self, data: &Vec<u32>) {
        self.write_int(data.len() as u32);
        for datum in data.to_vec().into_iter() {
            self.write_int(datum);
        }
    }

    /// Write long_array or vec<u64>
    pub fn write_long_array(&mut self, data: &Vec<u64>) {
        self.write_int(data.len() as u32);
        for datum in data.to_vec().into_iter() {
            self.write_long(datum);
        }
    }

    /// Write float or f32
    pub fn write_float(&mut self, data: f32) {
        self.write_raw(data.to_be_bytes().as_slice());

    }

    /// Write double or f64
    pub fn write_double(&mut self, data: f64) {
        self.write_raw(data.to_be_bytes().as_slice());
    }

    /// Write float_array or vec<f32>
    pub fn write_float_array(&mut self, data: &Vec<f32>) {
        self.write_int(data.len() as u32);
        for datum in data.to_vec().into_iter() {
            self.write_float(datum);
        }
    }

    /// Write double_array or vec<f64>
    pub fn write_double_array(&mut self, data: &Vec<f64>) {
        self.write_int(data.len() as u32);
        for datum in data.to_vec().into_iter() {
            self.write_double(datum);
        }
    }

    /// Write char
    pub fn write_char(&mut self, data: char) {
        self.write_byte(data as u8);
    }

    /// Write String
    pub fn write_string(&mut self, data: &String) {
        self.write_byte_array(&data.as_bytes().to_vec());
    }

    /// Write with given type and data list
    pub fn parse_types(&mut self, types: Vec<&'static str>, data: Vec<Box<dyn Any>>) {
        let data = &data;
        for (index, type_name) in types.into_iter().enumerate() {
            match type_name {
                "String" => self.write_string(data.as_slice()[index].downcast_ref::<String>().unwrap()),
                "char" => self.write_char(*data.as_slice()[index].downcast_ref::<char>().unwrap()),
                "i8" | "u8" => self.write_byte(*data.as_slice()[index].downcast_ref::<u8>().unwrap()),
                "i16" | "u16" => self.write_short(*data.as_slice()[index].downcast_ref::<u16>().unwrap()),
                "i32" | "u32" => self.write_int(*data.as_slice()[index].downcast_ref::<u32>().unwrap()),
                "i64" | "u64" => self.write_long(*data.as_slice()[index].downcast_ref::<u64>().unwrap()),
                "f32" => self.write_float(*data.as_slice()[index].downcast_ref::<f32>().unwrap()),
                "f64" => self.write_double(*data.as_slice()[index].downcast_ref::<f64>().unwrap()),
                "Vec<u8>" => self.write_byte_array(data.as_slice()[index].downcast_ref::<Vec<u8>>().unwrap()),
                "Vec<u16>" => self.write_short_array(data.as_slice()[index].downcast_ref::<Vec<u16>>().unwrap()),
                "Vec<u32>" => self.write_int_array(data.as_slice()[index].downcast_ref::<Vec<u32>>().unwrap()),
                "Vec<u64>" => self.write_long_array(data.as_slice()[index].downcast_ref::<Vec<u64>>().unwrap()),
                "Vec<f32>" => self.write_float_array(data.as_slice()[index].downcast_ref::<Vec<f32>>().unwrap()),
                "Vec<f64>" => self.write_double_array(data.as_slice()[index].downcast_ref::<Vec<f64>>().unwrap()),
                "Vec<i8>" => self.write_byte_array(&data.as_slice()[index].downcast_ref::<Vec<i8>>().unwrap().to_vec().into_iter().map(|x| x as u8).collect::<Vec<u8>>()),
                "Vec<i16>" => self.write_short_array(&data.as_slice()[index].downcast_ref::<Vec<i16>>().unwrap().to_vec().into_iter().map(|x| x as u16).collect::<Vec<u16>>()),
                "Vec<i32>" => self.write_int_array(&data.as_slice()[index].downcast_ref::<Vec<i32>>().unwrap().to_vec().into_iter().map(|x| x as u32).collect::<Vec<u32>>()),
                "Vec<i64>" => self.write_long_array(&data.as_slice()[index].downcast_ref::<Vec<i64>>().unwrap().to_vec().into_iter().map(|x| x as u64).collect::<Vec<u64>>()),
                _ => panic!("Unsupported Type, {}", type_name)
            }
        }
    }

    /// Write to the buffer, automatically extending the buffer if required
    pub fn write_raw(&mut self, data: &[u8]) {
        for i in 0 .. data.len() {
            if self.position + (i as i32) >= self.data.len() as i32 {
                self.data.push(0);
            }
            self.data[(self.position) as usize] = data[i];
            self.position += 1;
        }
    }
}