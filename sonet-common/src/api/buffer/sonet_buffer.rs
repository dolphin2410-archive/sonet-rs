use crate::api::buffer::buffer::*;
use crate::api::util::java_types::*;

pub struct SonetBuffer<T: Buffer> {
    data: T
}

impl <T: Buffer> SonetBuffer<T> {

    pub fn new_fixed(capacity: Int) -> SonetBuffer<FixedBuffer> {
        SonetBuffer { data: FixedBuffer::new(capacity) }
    }

    pub fn new_dynamic() -> SonetBuffer<DynamicBuffer> {
        SonetBuffer { data: DynamicBuffer::new() }
    }

    pub fn convert_unsigned(&mut self) -> &IntArray {
        unsafe { &*(self.data.as_slice() as *const _  as *const IntArray) }
    }

    pub fn write_int(&mut self, i: Int) {
        for b in i.to_be_bytes() {
            self.data.write(b);
        }
    }

    pub fn write_short(&mut self, s: Short) {
        for b in s.to_be_bytes() {
            self.data.write(b);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_short(bytes.len() as Short);
        for byte in bytes {
            self.data.write(*byte);
        }
    }

    pub fn write_char(&mut self, c: Char) {
        for b in c.to_string().as_bytes() {
            self.data.write(*b)
        }
    }

    pub fn write_boolean(&mut self, b: Boolean) {
        self.data.write(if b { 1 } else { 0 })
    }

    pub fn write_long(&mut self, l: Long) {
        for b in l.to_be_bytes() {
            self.data.write(b);
        }
    }

    pub fn write_float(&mut self, f: Float) {
        for b in f.to_be_bytes() {
            self.data.write(b);
        }
    }

    pub fn write_double(&mut self, d: Double) {
        for b in d.to_be_bytes() {
            self.data.write(b);
        }
    }

    pub fn read_int(&mut self) -> Int {
        let mut b_arr: [UnsignedByte; 4] = [0; 4];
        for i in 0..4 {
            b_arr[i] = self.data.read().unwrap()
        }
        Int::from_be_bytes(b_arr)
    }

    pub fn read_short(&mut self) -> Short {
        let mut b_arr: [UnsignedByte; 2] = [0; 2];
        for i in 0..2 {
            b_arr[i] = self.data.read().unwrap()
        }
        Short::from_be_bytes(b_arr)
    }

    pub fn read_string(&mut self) -> String {
        let size = self.read_short();
        let mut byte_arr: Vec<UnsignedByte> = Vec::new();
        for _ in 0..size {
            byte_arr.push(self.data.read().unwrap());
        }
        String::from_utf8(byte_arr).unwrap()
    }

    pub fn read_float(&mut self) -> Float {
        let mut b_arr: [UnsignedByte; 4] = [0; 4];
        for i in 0..4 {
            b_arr[i] = self.data.read().unwrap()
        }
        Float::from_be_bytes(b_arr)
    }

    pub fn read_long(&mut self) -> Long {
        let mut b_arr: [UnsignedByte; 8] = [0; 8];
        for i in 0..8 {
            b_arr[i] = self.data.read().unwrap()
        }
        Long::from_be_bytes(b_arr)
    }

    pub fn read_char(&mut self) -> Char {
        self.read_string().chars().next().unwrap()
    }

    pub fn read_boolean(&mut self) -> Boolean {
        let byte = self.data.read().unwrap();

        if byte == 1 { true } else { false }
    }

    pub fn read_double(&mut self) -> Double {
        let mut b_arr: [UnsignedByte; 8] = [0; 8];
        for i in 0..8 {
            b_arr[i] = self.data.read().unwrap()
        }
        Double::from_be_bytes(b_arr)
    }
}