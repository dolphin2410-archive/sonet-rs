use crate::api::util::java_types::*;

pub trait Buffer {
    fn as_slice(&mut self) -> &[UnsignedByte];
    fn read(&mut self) -> Result<UnsignedByte, &'static str>;
    fn write(&mut self, data: UnsignedByte);
    fn reset(&mut self);
}

pub struct FixedBuffer {
    pub capacity: Int,
    data: Vec<UnsignedByte>,
    position: Int
}

pub struct DynamicBuffer {
    data: Vec<UnsignedByte>,
    position: Int
}

impl FixedBuffer {
    pub fn new(capacity: Int) -> FixedBuffer {
        FixedBuffer { capacity, data: Vec::new(), position: 0 }
    }
}

impl Buffer for FixedBuffer {
    fn as_slice(&mut self) -> &[UnsignedByte] {
        self.data.as_slice()
    }

    fn read(&mut self) -> Result<UnsignedByte, &'static str> {
        match self.data.get((self.position + 1) as usize) {
            Some(val) => {
                self.position += 1;
                Ok(*val)
            },
            None => Err("Error")
        }
    }

    fn write(&mut self, data: UnsignedByte) {
        if self.position <= self.capacity {
            self.data.insert(self.position as usize, data)
        }
    }

    fn reset(&mut self) {
        self.position = 0;
    }
}

impl DynamicBuffer {
    pub fn new() -> DynamicBuffer {
        DynamicBuffer { data: Vec::new(), position: 0 }
    }
}

impl Buffer for DynamicBuffer {
    fn as_slice(&mut self) -> &[UnsignedByte] {
        self.data.as_slice()
    }


    fn read(&mut self) -> Result<UnsignedByte, &'static str> {
        match self.data.get((self.position + 1) as usize) {
            Some(val) => {
                self.position += 1;
                Ok(*val)
            },
            None => Err("Error")
        }
    }

    fn write(&mut self, data: UnsignedByte) {
        self.data.insert(self.position as usize, data)
    }

    fn reset(&mut self) {
        self.position = 0;
    }
}

pub struct BufUnderFlow;
pub struct BufOverFlow;