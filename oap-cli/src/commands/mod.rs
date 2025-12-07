use std::path::PathBuf;

pub struct Context {
    pub json: bool,
    pub verbose: bool,
    pub config: Option<PathBuf>,
}

pub mod did;
pub mod relay;
pub mod msg;
pub mod connect;
pub mod listen;
pub mod send;
