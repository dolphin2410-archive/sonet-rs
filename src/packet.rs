use std::any::Any;
use std::collections::HashMap;

/// Packet Trait. Can be serialized and deserialized from and into TCP Packets
pub trait Packet {

    /// Gets self as the Any type
    fn as_any(&self) -> &dyn Any;

    /// Gets the packet's name
    fn get_name(&self) -> &'static str;

    /// Gets the field names of the Packet Struct
    fn object_field_names(&self) -> Vec<&'static str>;

    /// Gets the field types of the Packet Struct
    fn object_type_names(&self) -> Vec<&'static str>;

    /// Gets the field values of the Packet Struct
    fn get_values(&self) -> Vec<Box<dyn std::any::Any>>;
}

/// PacketWrapper contains a supplier that generates a packet instance with Vector parameters
pub struct PacketWrapper {

    /// Supplier for fields
    fields_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>,

    /// Supplier for instances
    instance_accessor: Option<Box<dyn Fn(Vec<Box<dyn Any>>) -> Box<dyn Packet>>>,

    /// Supplier for types
    types_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>
}

/// Default PacketWrapper Implementation
impl PacketWrapper {

    /// Creates a new wrapper instance
    pub fn new(
        fields_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>,
        instance_accessor: Option<Box<dyn Fn(Vec<Box<dyn Any>>) -> Box<dyn Packet>>>,
        types_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>) -> Self {
        Self {
            fields_accessor,
            instance_accessor,
            types_accessor
        }
    }

    /// Generate instance with the supplier
    pub fn create_instance(&self, data: Vec<Box<dyn Any>>) -> Box<dyn Packet> {
        let boxed_packet: Box<dyn Packet> = self.instance_accessor.as_ref().unwrap()(data);
        boxed_packet
    }

    /// Get the field names with the supplier
    pub fn get_fields(&self) -> Vec<&'static str> {
        let fields: Vec<&'static str> = self.fields_accessor.as_ref().unwrap()();
        fields
    }

    /// Get the field types with the supplier
    pub fn get_types(&self) -> Vec<&'static str> {
        let types: Vec<&'static str> = self.types_accessor.as_ref().unwrap()();
        types
    }
}

/// PacketRegistry. Contains data of the registered packets
pub struct PacketRegistry {

    /// The data of the packets
    pub keys: HashMap<String, PacketWrapper>,
}

/// Default PacketRegistry Implementation
impl PacketRegistry {

    /// New Registry
    pub fn new() -> Self {
        Self {
            keys: HashMap::new()
        }
    }

    /// Register a packet
    pub fn register(&mut self, name: String, wrapper: PacketWrapper) {
        self.keys.insert(name, wrapper);
    }
}

#[macro_export]
/// Creates a Packet implementation easily.
///
/// eg )
/// ```rust
/// packet! {
///     @jvm("io.github.dolphin2410.packets.EntitySpawnPacket")
///     EntitySpawnRustPacket {
///         entity_id: i32,
///         entity_name: String,
///         network_connection_id: u8
///     }
/// }
/// ```
///
/// With the following code, Sonet will map all the packets named "io.github.dolphin2410.packets.EntitySpawnPacket" to the struct EntitySpawnRustPacket. For Jvm compatibility, the names will default to Java's package-class names
macro_rules! packet {
    (@jvm($jvmname:literal) $name:ident { $($fname:ident : $ftype:ty),* }) => {

        #[derive(Clone)]
        pub struct $name {
            pub $($fname : $ftype),*
        }

        impl sonet_rs::packet::Packet for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_name(&self) -> &'static str {
                $jvmname
            }

            fn object_field_names(&self) -> Vec<&'static str> {
                vec![$(stringify!($fname)),*]
            }

            fn object_type_names(&self) -> Vec<&'static str> {
                vec![$(stringify!($ftype)),*]
            }

            fn get_values(&self) -> Vec<Box<dyn std::any::Any>> {
                let mut values: Vec<Box<dyn std::any::Any>> = vec![];

                $(values.push(Box::new(self.$fname.clone()));)*

                values
            }
        }

        impl $name {

            pub fn field_names() -> Vec<&'static str> {
                vec![$(stringify!($fname)),*]
            }

            pub fn type_names() -> Vec<&'static str> {
                vec![$(stringify!($ftype)),*]
            }

            pub fn new(vec: Vec<Box<dyn Any>>) -> Self {
                let fields = Self::field_names();
                let mut iterator = sonet_rs::util::JIterator::new(vec);
                Self {
                    $($fname : (*iterator.next()).downcast_ref::<$ftype>().unwrap().to_owned() ),*
                }
            }

            fn jvm_name() -> &'static str {
                $jvmname
            }

            pub fn register(registry: &mut sonet_rs::packet::PacketRegistry) {
                let mut wrapper = sonet_rs::packet::PacketWrapper::new(
                    Some(Box::new(||{
                        Self::field_names()
                    })),
                    Some(Box::new(|vec|{
                        Box::new(Self::new(vec))
                    })),
                    Some(Box::new(||{
                        Self::type_names()
                    })));

                registry.register($jvmname.to_string(), wrapper);
            }
        }
    };
}