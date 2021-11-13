use crate::packets::*;

pub struct MyPacket {

}

impl Packet for MyPacket {
    fn send(&self) {
        println!("SENT");
    }
}