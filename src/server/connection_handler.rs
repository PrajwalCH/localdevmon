mod request;

use std::io::{self, Read};
use std::net::TcpStream;

use super::route_handler::DirNode;
use request::HTTPRequest;

pub struct ConnectionHandler<'a> {
    dir_root_node: &'a DirNode,
    stream: TcpStream,
    request: HTTPRequest,
}

impl<'a> ConnectionHandler<'a> {
    pub fn new(mut stream: TcpStream, dir_root_node: &'a DirNode) -> io::Result<Self> {
        let mut buf = [0; 1024];
        let num_bytes_read = stream.read(&mut buf)?;
        let request = HTTPRequest::parse(&buf[..num_bytes_read]);

        Ok(Self {
            dir_root_node,
            stream,
            request,
        })
    }

    fn log_request(request_obj: &HTTPRequest) {
        println!(
            "\x1b[1;32m[request]\x1b[0m {} {} HTTP/{}",
            request_obj.method, request_obj.path, request_obj.version
        );
    }
}
