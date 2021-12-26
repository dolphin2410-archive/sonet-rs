pub struct Buffer {
    data: Vec<u8>,
    position: i32
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    pub fn read_int(&mut self) -> i32 {
        let mut fixed = [0_u8; 4];
        fixed.copy_from_slice(&self.data[self.position as usize .. (self.position + 4) as usize]);
        self.position += 4;
        i32::from_be_bytes(fixed)
    }
}