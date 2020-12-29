meta!();

#[macro_export]
macro_rules! meta {
    () => {
        pub mod meta {
            pub static VERSION: &str = env!("CARGO_PKG_VERSION");
            pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
            pub static NAME: &str = env!("CARGO_PKG_NAME");
            pub static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
        }
    };
}
