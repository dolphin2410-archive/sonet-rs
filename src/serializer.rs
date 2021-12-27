use crate::buffer::read::SonetReadBuf;
use crate::packet::{Packet, PacketRegistry};

pub struct Serializer {
    pub registry: PacketRegistry,
}

impl Serializer {
    pub fn new(registry: PacketRegistry) -> Self {
        Self { registry }
    }

    pub fn serialize(&self, packet: Box<impl Packet>) -> SonetReadBuf {
        // let name = packet.get_name();

        // let fields = packet.object_field_names();
        // let types = packet.object_type_names();

        let data = packet.get_values();

        for datum in data {
            println!("{}", datum.downcast_ref::<String>().unwrap());
        }

        SonetReadBuf::new(vec![])
    }

    pub fn deserialize(&self, buffer: &mut SonetReadBuf) -> Box<dyn Packet> {
        let packet_name = buffer.read_string();

        let packet_wrapper = &self.registry.keys.get(&packet_name).unwrap();

        let types = packet_wrapper.get_types();
        let data = buffer.parse_types(types);
        packet_wrapper.create_instance(data)
    }
}