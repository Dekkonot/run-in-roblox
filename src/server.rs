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

pub fn process_requests(server: Server, server_id: Uuid) -> Result<Vec<(MessageType, String)>> {
    let mut result = Vec::new();
    loop {
        // TODO: Add threadpool, and a timeout
        let mut request = server.recv()?;
        log::debug!("request: {} {}", request.method(), request.url());
        match (request.method(), request.url()) {
            (Method::Get, "/stop") => {
                request.respond(Response::empty(200))?;
                return Ok(result);
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
            (Method::Post, "/info") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                result.push((MessageType::Info, text));

                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/output") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                result.push((MessageType::Output, text));

                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/warn") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                result.push((MessageType::Warn, text));

                request.respond(Response::empty(200))?;
            }
            (Method::Post, "/error") => {
                let mut text = String::with_capacity(request.body_length().unwrap_or(0));
                request.as_reader().read_to_string(&mut text)?;
                result.push((MessageType::Error, text));

                request.respond(Response::empty(200))?;
            }
            (method, url) => {
                log::error!("unknown request to server: {method} {url}")
            }
        }
    }
}
