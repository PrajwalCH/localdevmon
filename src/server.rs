mod map_route;

use std::env;
use std::io::{self, Read};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;

use map_route::*;

#[derive(Debug)]
pub struct ServerConfig {
    pub port_num: u16,
    pub host_addr: Ipv4Addr,
    pub path: PathBuf,
}

#[derive(Debug)]
struct HTTPRequest {
    method: String,
    path: String,
    version: String,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            port_num: 8000,
            host_addr: Ipv4Addr::new(127, 0, 0, 1),
            path: env::current_dir().unwrap_or(PathBuf::from(".")),
        }
    }
}

pub fn start(server_config: ServerConfig) -> io::Result<()> {
    let routes = map_route(&server_config.path)?;
    let listener = tcp_listen(server_config.host_addr, server_config.port_num)?;

    for stream in listener.incoming() {
        let stream = stream?;

        println!("Connection established");
        handle_connection(stream);
    }

    Ok(())
}

fn tcp_listen(host_addr: Ipv4Addr, port_num: u16) -> io::Result<TcpListener> {
    let sock_addr = SocketAddr::from((host_addr, port_num));
    let listener = TcpListener::bind(sock_addr)?;

    println!("Server listening on: http://localhost:{}", port_num);

    Ok(listener)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = parse_request(&buffer);
    println!("{:#?}", request);
}

fn parse_request(buffer: &[u8]) -> HTTPRequest {
    let (request_line, _request_line_end) = parse_request_line(&buffer);
    // TODO: implement headers parser if needed
    //let (_, headers) = buffer.split_at(request_line_end);
    println!("{:?}", request_line);

    HTTPRequest {
        method: String::from_utf8_lossy(&request_line[0][..]).into_owned(),
        path: String::from_utf8_lossy(&request_line[1][..]).into_owned(),
        version: parse_http_version(&request_line[2]),
    }
}

fn parse_request_line(buffer: &[u8]) -> (Vec<Vec<u8>>, usize) {
    let mut request_line: Vec<u8> = Vec::new();
    let mut request_line_end = 0;

    for (idx, byte) in buffer.iter().enumerate() {
        if *byte == b'\n' {
            request_line_end = idx;
            // remove previously pushed '\r' and stop the loop
            request_line.pop();
            break;
        }
        request_line.push(*byte);
    }

    let request_line: Vec<Vec<u8>> = request_line
        .split(|byte| *byte == b' ')
        .map(|token| token.to_vec()) // convert &[u8] to owned Vec<u8>
        .collect();

    (request_line, request_line_end)
}

fn parse_http_version(buffer: &[u8]) -> String {
    if !buffer.starts_with(b"HTTP/") {
        return "0.1".to_string();
    }

    let (_, version) = buffer.split_at(5);

    String::from_utf8_lossy(&version).into_owned()
}
