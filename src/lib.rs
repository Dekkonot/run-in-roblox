pub mod cli;
mod error;
mod injection;
pub mod roblox;
pub mod server;

pub use error::{Error, Result};

/// If not port is specified, this port is used for the webserver.
pub const DEFAULT_PORT: u16 = 34871;
