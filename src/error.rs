use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown script type provided. valid values: plugin, server, client")]
    UnknownScriptType,

    #[error("the folder that Roblox Studio uses for local plugins does not exist")]
    PluginFolderNoExists,

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("rbxl serializing error: {0}")]
    RbxBinaryEncode(#[from] rbx_binary::EncodeError),
}
