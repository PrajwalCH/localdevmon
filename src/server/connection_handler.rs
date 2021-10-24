use std::io::{self, Read};
use std::net::TcpStream;
use std::path::PathBuf;

use super::route_handler::DirNode;
use request::HTTPRequest;

type LoggerFn = fn(&HTTPRequest);

pub struct ConnectionHandler<'a> {
    dir_root_node: &'a DirNode,
    stream: TcpStream,
    request: HTTPRequest,
    request_logger: Option<LoggerFn>,
}

impl<'a> ConnectionHandler<'a> {
    pub fn new(mut stream: TcpStream, dir_root_node: &'a DirNode) -> io::Result<Self> {
        let mut buf = [0; 1024];
        stream.read(&mut buf)?;
        let request = HTTPRequest::parse(&buf);

        Ok(Self {
            dir_root_node,
            stream,
            request,
        })
    }
}
