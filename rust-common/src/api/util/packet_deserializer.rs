use std::str::from_utf8;

pub struct PacketDeserializer {

}

impl PacketDeserializer {
    // i32 is compatible to java's 'int' type
    pub fn convert_string(s: &str) -> i32 {
        todo!()
    }

    pub fn convert_boolean(b: bool) -> i32 {
        match b {
            true => 1,
            false => 0
        }
    }
}