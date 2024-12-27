use clap::Parser;

use rir::cli::{color_to_write_style, CliOptions};

fn main() {
    let cli = CliOptions::parse();

    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None)
        .format_indent(Some(8))
        .write_style(color_to_write_style(cli.color))
        .init();

    let _ = cli.run().unwrap();
}
