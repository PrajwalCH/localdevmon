use std::io::{self, Read};
use std::net::TcpStream;
use std::path::PathBuf;

use super::request::HTTPRequest;

type LoggerFn = fn(&HTTPRequest);

pub struct ConnectionHandler<'a> {
    routes: &'a [PathBuf],
    stream: TcpStream,
    request: HTTPRequest,
    request_logger: Option<LoggerFn>,
}

impl<'a> ConnectionHandler<'a> {
    pub fn new(
        mut stream: TcpStream,
        routes: &'a [PathBuf],
        request_logger: Option<LoggerFn>,
    ) -> io::Result<Self> {
        let mut buf = [0; 1024];
        stream.read(&mut buf)?;
        let request = HTTPRequest::parse(&buf);

        Ok(Self {
            routes,
            stream,
            request,
            request_logger,
        })
    }
}
