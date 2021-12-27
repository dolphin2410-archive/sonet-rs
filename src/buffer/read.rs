use std::any::Any;

/// SonetReadBuf
pub struct SonetReadBuf {
    data: Vec<u8>,
    position: i32
}

/// Default SonetReadBuf Implementation
impl SonetReadBuf {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    pub fn read_byte(&mut self) -> u8 {
        let mut fixed = [0_u8; 1];
        fixed.copy_from_slice(&self.data[self.position as usize .. (self.position + 1) as usize]);
        self.position += 1;
        u8::from_be_bytes(fixed)
    }

    pub fn read_short(&mut self) -> u16 {
        let mut fixed = [0_u8; 2];
        fixed.copy_from_slice(&self.data[self.position as usize .. (self.position + 2) as usize]);
        self.position += 2;
        u16::from_be_bytes(fixed)
    }

    pub fn read_int(&mut self) -> u32 {
        let mut fixed = [0_u8; 4];
        fixed.copy_from_slice(&self.data[self.position as usize .. (self.position + 4) as usize]);
        self.position += 4;
        u32::from_be_bytes(fixed)
    }

    pub fn read_long(&mut self) -> u64 {
        let mut fixed = [0_u8; 8];
        fixed.copy_from_slice(&self.data[self.position as usize .. (self.position + 8) as usize]);
        self.position += 8;
        u64::from_be_bytes(fixed)
    }

    pub fn read_bool(&mut self) -> bool {
        match self.read_byte() {
            0 => false,
            1 => true,
            _ => panic!("Invalid Boolean")
        }
    }

    pub fn read_byte_array(&mut self) -> Vec<u8> {
        let size = self.read_int();
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(self.read_byte())
        }
        vec
    }

    pub fn read_short_array(&mut self) -> Vec<u16> {
        let size = self.read_int();
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(self.read_short())
        }
        vec
    }

    pub fn read_int_array(&mut self) -> Vec<u32> {
        let size = self.read_int();
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(self.read_int())
        }
        vec
    }

    pub fn read_long_array(&mut self) -> Vec<u64> {
        let size = self.read_int();
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(self.read_long())
        }
        vec
    }

    pub fn read_float(&mut self) -> f32 {
        let mut fixed = [0_u8; 4];
        fixed.copy_from_slice(&self.data[self.position as usize .. (self.position + 4) as usize]);
        self.position += 4;
        f32::from_be_bytes(fixed)
    }

    pub fn read_double(&mut self) -> f64 {
        let mut fixed = [0_u8; 8];
        fixed.copy_from_slice(&self.data[self.position as usize .. (self.position + 8) as usize]);
        self.position += 8;
        f64::from_be_bytes(fixed)
    }

    pub fn read_float_array(&mut self) -> Vec<f32> {
        let size = self.read_int();
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(self.read_float())
        }
        vec
    }

    pub fn read_double_array(&mut self) -> Vec<f64> {
        let size = self.read_int();
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(self.read_double())
        }
        vec
    }

    pub fn read_char(&mut self) -> char {
        char::from(self.read_byte())
    }

    pub fn read_string(&mut self) -> String {
        let size = self.read_int();
        let mut str = String::new();
        for _ in 0..size {
            str.push(self.read_char());
        }
        str
    }

    pub fn parse_types(&mut self, types: Vec<&'static str>) -> Vec<Box<dyn Any>> {
        let mut vec: Vec<Box<dyn Any>> = vec![];
        for type_name in types {
            match type_name {
                "String" => vec.push(Box::new(self.read_string())),
                "char" => vec.push(Box::new(self.read_char())),
                "u8" => vec.push(Box::new(self.read_byte())),
                "u16" => vec.push(Box::new(self.read_short())),
                "u32" => vec.push(Box::new(self.read_int())),
                "u64" => vec.push(Box::new(self.read_long())),
                "f32" => vec.push(Box::new(self.read_float())),
                "f64" => vec.push(Box::new(self.read_double())),
                "i8" => vec.push(Box::new(self.read_byte() as i8)),
                "i16" => vec.push(Box::new(self.read_short() as i16)),
                "i32" => vec.push(Box::new(self.read_int() as i32)),
                "i64" => vec.push(Box::new(self.read_long() as i64)),
                "Vec<u8>" => vec.push(Box::new(self.read_byte_array())),
                "Vec<u16>" => vec.push(Box::new(self.read_short_array())),
                "Vec<u32>" => vec.push(Box::new(self.read_int_array())),
                "Vec<u64>" => vec.push(Box::new(self.read_long_array())),
                "Vec<f32>" => vec.push(Box::new(self.read_float_array())),
                "Vec<f64>" => vec.push(Box::new(self.read_double_array())),
                "Vec<i8>" => vec.push(Box::new(self.read_byte_array().into_iter().map(|x| x as i8).collect::<Vec<i8>>())),
                "Vec<i16>" => vec.push(Box::new(self.read_short_array().into_iter().map(|x| x as i16).collect::<Vec<i16>>())),
                "Vec<i32>" => vec.push(Box::new(self.read_int_array().into_iter().map(|x| x as i32).collect::<Vec<i32>>())),
                "Vec<i64>" => vec.push(Box::new(self.read_long_array().into_iter().map(|x| x as i64).collect::<Vec<i64>>())),
                _ => panic!("Unsupported Type, {}", type_name)
            }
        }
        vec
    }
}