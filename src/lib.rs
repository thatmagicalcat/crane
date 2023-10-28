pub mod response;
pub mod webserver;

pub type Query = std::collections::HashMap<String, Vec<String>>;

// TODO: use environment variable
pub const NUM_THREADS: u32 = 4;
