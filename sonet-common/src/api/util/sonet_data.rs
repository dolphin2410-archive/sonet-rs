#[macro_export]
macro_rules! sonet_data {
    (struct $name:ident { $($key:ident : $value:ty ),* }) => {
        struct $name {

        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                &[$(stringify!($key)),*]
            }
        }
    };
}