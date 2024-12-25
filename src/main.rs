use std::{net::Ipv4Addr, num::NonZeroU16};

use clap::Parser;

use rir::cli::{color_to_write_style, CliOptions};

use tiny_http::{Response, Server};

fn main() {
    let cli = CliOptions::parse();

    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None)
        .format_indent(Some(8))
        .write_style(color_to_write_style(cli.color))
        .init();

    let addr = (
        Ipv4Addr::from([0, 0, 0, 0]),
        cli.port.map(NonZeroU16::get).unwrap_or(0),
    );
    let server = Server::http(addr).unwrap();
    log::info!("Listening on server: {}", server.server_addr());

    loop {
        let request = server.recv().unwrap();
        log::debug!("{} {}", request.method(), request.url());
        request.respond(Response::empty(200)).unwrap();
    }
}
