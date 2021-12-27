use crate::buffer::read::SonetReadBuf;
use crate::buffer::write::SonetWriteBuf;
use crate::packet::{Packet, PacketRegistry};

/// The Serializer & Deserializer for a packet
pub struct Codec {

    /// The packet registry
    pub registry: PacketRegistry,
}

impl Codec {

    /// Creates a new Codec instance
    pub fn new(registry: PacketRegistry) -> Self {
        Self { registry }
    }

    /// Serializes the packet object into a ReadBuffer
    pub fn serialize(&self, packet: Box<impl Packet>) -> SonetReadBuf {
        // Name of the packet
        let name = packet.get_name();

        // The types of the packet struct fields
        let types = packet.object_type_names();

        // The packet struct field values
        let data = packet.get_values();

        // New WriteBuf
        let mut buf = SonetWriteBuf::new();

        // Write the name to buffer
        buf.write_string(&name.to_string());

        // Write the values automatically
        buf.parse_types(types, data);

        // Convert WriteBuf -> ReadBuf
        buf.readable()
    }

    /// Deserializes the ReadBuffer into a packet instance
    pub fn deserialize(&self, buffer: &mut SonetReadBuf) -> Box<dyn Packet> {
        // Read the packet's name
        let packet_name = buffer.read_string();

        // Get the instance-creation supplier from the registry.
        let packet_wrapper = &self.registry.keys.get(&packet_name).unwrap();

        // Get the packet's struct field types
        let types = packet_wrapper.get_types();

        // Read the struct field values from ReadBuf
        let data = buffer.parse_types(types);

        // Create instance with the value given
        packet_wrapper.create_instance(data)
    }
}