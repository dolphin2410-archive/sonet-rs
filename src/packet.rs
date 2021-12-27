use std::any::Any;
use std::collections::HashMap;

pub trait Packet {
    fn as_any(&self) -> &dyn Any;

    fn get_name(&self) -> &'static str;

    fn object_field_names(&self) -> Vec<&'static str>;

    fn object_type_names(&self) -> Vec<&'static str>;

    fn get_values(&self) -> Vec<Box<dyn std::any::Any>>;
}

pub struct PacketWrapper {
    fields_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>,
    instance_accessor: Option<Box<dyn Fn(Vec<Box<dyn Any>>) -> Box<dyn Packet>>>,
    types_accessor: Option<Box<dyn Fn() -> Vec<&'static str>>>
}

impl PacketWrapper {
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

    pub fn new_empty() -> Self {
        Self {
            fields_accessor: None,
            instance_accessor: None,
            types_accessor: None
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

    pub fn get_types(&self) -> Vec<&'static str> {
        let types: Vec<&'static str> = self.types_accessor.as_ref().unwrap()();
        types
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

                registry.add_entry($jvmname.to_string(), wrapper);
            }
        }
    };
}