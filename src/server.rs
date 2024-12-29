use tiny_http::{Method, Response, Server, StatusCode};
use uuid::Uuid;

use crate::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageType {
    Info,
    Output,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerOutput {
    pub success: bool,
    pub messages: Vec<(MessageType, String)>,
}

pub fn process_requests(server: Server, server_id: Uuid) -> Result<ServerOutput> {
    let mut messages = Vec::new();
    let mut success = true;
    loop {
        // TODO: Add threadpool, and a timeout
        let mut request = server.recv()?;
        log::debug!("request: {} {}", request.method(), request.url());
        match (request.method(), request.url()) {
            (Method::Get, "/stop") => {
                request.respond(Response::empty(200))?;
                return Ok(ServerOutput { messages, success });
            }
            (Method::Get, "/id") => {
                request.respond(Response::new(
                    StatusCode(200),
                    Vec::new(),
                    server_id.to_string().as_bytes(),
                    None,
                    None,
                ))?;
            }
            (Method::Post, "/status/ok") => {
                success = true;
                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/status/error") => {
                success = false;
                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/info") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                messages.push((MessageType::Info, text));

                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/output") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                messages.push((MessageType::Output, text));

                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/warn") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                messages.push((MessageType::Warn, text));

                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/error") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                messages.push((MessageType::Error, text));

                request.respond(Response::empty(200))?;
            }
            (method, url) => {
                log::error!("unknown request to server: {method} {url}");
                request.respond(Response::empty(400))?;
            }
        }
    }
}
