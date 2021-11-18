use crate::api::packets::Packet;

pub trait PacketHandler: Send {
    fn handle(&self, packet: dyn Packet);
}