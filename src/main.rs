use std::process::{self, ExitCode};

use clap::{ColorChoice, Parser};

use env_logger::WriteStyle;
use owo_colors::{OwoColorize, Stream};
use run_in_roblox::{
    cli::CliOptions,
    server::{MessageType, ServerOutput},
};

fn main() -> ExitCode {
    let cli = CliOptions::parse();
    let write_style = match cli.color {
        ColorChoice::Auto => WriteStyle::Auto,
        ColorChoice::Always => {
            owo_colors::set_override(true);
            WriteStyle::Always
        }
        ColorChoice::Never => {
            owo_colors::set_override(false);
            WriteStyle::Never
        }
    };

    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None)
        .format_indent(Some(8))
        .write_style(write_style)
        .init();

    let ServerOutput { messages, success } = cli.run().unwrap();
    for (message_type, message) in messages {
        match message_type {
            MessageType::Info => println!(
                "{}",
                message.if_supports_color(Stream::Stdout, |text| text.cyan())
            ),
            MessageType::Output => println!(
                "{}",
                message.if_supports_color(Stream::Stdout, |text| text.default_color())
            ),
            MessageType::Warn => println!(
                "{}",
                message.if_supports_color(Stream::Stdout, |text| text.yellow())
            ),
            MessageType::Error => println!(
                "{}",
                message.if_supports_color(Stream::Stdout, |text| text.red())
            ),
        }
    }

    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
