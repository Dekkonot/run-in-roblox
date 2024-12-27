use std::{env::temp_dir, net::Ipv4Addr, num::NonZeroU16, path::PathBuf, str::FromStr};

use clap::{builder::PossibleValue, ColorChoice, Parser, ValueEnum};
use env_logger::fmt;
use tiny_http::Server;
use uuid::Uuid;

use crate::{
    injection, roblox,
    server::{process_requests, MessageType},
    Error, Result,
};

#[derive(Debug, Parser)]
pub struct CliOptions {
    /// The path to the script that should be ran.
    #[clap(long, short)]
    pub script: PathBuf,
    /// If provided, should be a link to an rbxm or rbxl file that the
    /// provided script will run inside of.
    #[clap(long, short)]
    pub place: Option<PathBuf>,

    /// Sets what port the tool's server runs on. If none is provided, it will
    /// default to one assigned by the system.
    #[clap(long)]
    pub port: Option<NonZeroU16>,

    // TODO: Support running non-plugin scripts using Studio's CLI flags
    // Indicates what type of script to run the provided file as.
    // #[clap(long, default_value("plugin"))]
    // pub kind: ScriptKind,
    /// Controls how color is displayed by this tool.
    #[clap(long, default_value("auto"))]
    pub color: ColorChoice,
}

impl CliOptions {
    pub fn run(self) -> Result<Vec<(MessageType, String)>> {
        let kind = ScriptKind::Plugin;
        let port = self.port.map(NonZeroU16::get).unwrap_or(0);

        let server_id = Uuid::new_v4();

        log::debug!("Bundling script as {kind:?}");
        match kind {
            ScriptKind::Plugin => {
                injection::bundle_plugin(&self.script, port, server_id)?;
            }
            _ => unimplemented!("script kind {kind:?} not supported yet"),
        }

        let addr = (Ipv4Addr::from([0, 0, 0, 0]), port);
        let server = Server::http(addr).unwrap();
        log::warn!("Listening on server: {}", server.server_addr());

        let place = match self.place {
            Some(place) => place,
            None => {
                let path = temp_dir().join("run-in-roblox-empty.rbxl");
                roblox::create_empty_place(&path)?;
                path
            }
        };

        let child = match kind {
            ScriptKind::Plugin => roblox::launch_studio_edit(&place)?,
            _ => unimplemented!("script kind {kind:?} not supported yet"),
        };

        let requests = process_requests(server, server_id)?;

        drop(child);

        Ok(requests)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScriptKind {
    Server,
    Client,
    Plugin,
}

impl FromStr for ScriptKind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "server" => Ok(Self::Server),
            "client" => Ok(Self::Client),
            "plugin" => Ok(Self::Plugin),
            _ => Err(Self::Err::UnknownScriptType),
        }
    }
}

impl ValueEnum for ScriptKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Plugin, Self::Server, Self::Client]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Plugin => PossibleValue::new("plugin"),
            Self::Server => PossibleValue::new("server"),
            Self::Client => PossibleValue::new("client"),
        })
    }
}

pub fn color_to_write_style(color: ColorChoice) -> fmt::WriteStyle {
    match color {
        ColorChoice::Auto => fmt::WriteStyle::Auto,
        ColorChoice::Always => fmt::WriteStyle::Always,
        ColorChoice::Never => fmt::WriteStyle::Never,
    }
}
