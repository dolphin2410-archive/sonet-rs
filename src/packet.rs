use std::any::Any;
use std::collections::HashMap;

pub trait Packet {
    fn as_any(&self) -> &dyn Any;
}

pub struct PacketWrapper {
    fields_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>,
    instance_accessor: Option<Box<dyn Fn(Vec<Box<dyn Any>>) -> Box<dyn Packet>>>
}

impl PacketWrapper {
    pub fn new(fields_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>, instance_accessor: Option<Box<dyn Fn(Vec<Box<dyn Any>>) -> Box<dyn Packet>>>) -> Self {
        Self {
            fields_accessor,
            instance_accessor
        }
    }

    pub fn new_empty() -> Self {
        Self {
            fields_accessor: None,
            instance_accessor: None
        }
    }

    pub fn create_instance(&self, data: Vec<Box<dyn Any>>) -> Box<dyn Packet> {
        let boxed_packet: Box<dyn Packet> = self.instance_accessor.as_ref().unwrap()(data);
        boxed_packet
    }

    pub fn get_fields(&self) -> Vec<&'static str> {
        let fields: Vec<&'static str> = self.fields_accessor.as_ref().unwrap()();
        fields
    }
}

pub struct PacketRegistry {
    pub keys: HashMap<String, PacketWrapper>,
}

impl PacketRegistry {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, name: String, wrapper: PacketWrapper) {
        self.keys.insert(name, wrapper);
    }
}

#[macro_export]
macro_rules! packet {
    ($name:ident { $($fname:ident : $ftype:ty),* }) => {
        pub struct $name {
            $($fname : $ftype),*
        }

        impl sonet_rs::packet::Packet for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl $name {

            pub fn field_names() -> Vec<&'static str> {
                vec![$(stringify!($fname)),*]
            }

            pub fn new(vec: Vec<Box<dyn Any>>) -> Self {
                let fields = Self::field_names();
                let mut iterator = sonet_rs::util::JIterator::new(vec);
                Self {
                    $($fname : (*iterator.next()).downcast_ref::<$ftype>().unwrap().to_owned() ),*
                }
            }

            pub fn register(registry: &mut sonet_rs::packet::PacketRegistry) {
                let mut wrapper = sonet_rs::packet::PacketWrapper::new(
                    Some(Box::new(||{
                        Self::field_names()
                    })),
                    Some(Box::new(|vec|{
                        Box::new(Self::new(vec))
                    })));

                registry.add_entry(stringify!($name).to_string(), wrapper);
            }
        }
    }
}