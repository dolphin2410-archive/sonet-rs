use crate::api::packets::Packet;
use crate::api::util::java_types::UnsignedByte;

pub struct PacketSerializer;

impl PacketSerializer {
    pub fn serialize(packet: Box<dyn Packet>) -> &'static [UnsignedByte] {
        todo!()
    }
}