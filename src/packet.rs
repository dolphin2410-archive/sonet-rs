use std::any::Any;
use std::collections::HashMap;

pub trait Packet {
    fn as_any(&self) -> &dyn Any;
}

pub struct Registry {
    pub map: HashMap<String, Box<dyn Fn(Vec<Box<dyn Any>>) -> Box<dyn Packet>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, name: String, closure: Box<dyn Fn(Vec<Box<dyn Any>>) -> Box<dyn Packet>>) {
        self.map.insert(name, closure);
    }
}

pub struct JIterator<T> {
    vec: Vec<T>,
    position: usize,
}

impl<T> JIterator<T> {
    pub fn new(vec: Vec<T>) -> Self {
        return Self {
            vec,
            position: 0,
        };
    }

    pub fn next(&mut self) -> &T {
        self.position += 1;
        &self.vec[self.position - 1]
    }
}

#[macro_export]
macro_rules! packet {
    ($name:ident { $($fname:ident : $ftype:ty),* }) => {
        pub struct $name {
            $($fname : $ftype),*
        }

        impl sonet_rs::packet::Packet for $name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        impl $name {

            pub fn field_names() -> Vec<&'static str> {
                vec![$(stringify!($fname)),*]
            }

            pub fn new(vec: Vec<Box<dyn Any>>) -> Self {
                let fields = Self::field_names();
                let mut iterator = sonet_rs::packet::JIterator::new(vec);
                Self {
                    $($fname : (*iterator.next()).downcast_ref::<$ftype>().unwrap().to_owned() ),*
                }
            }

            pub fn register(registry: &mut sonet_rs::packet::Registry) {
                registry.add_entry(stringify!($name).to_string(), Box::new(|vec|{
                    Box::new(Self::new(vec))
                }));
            }
        }
    }
}