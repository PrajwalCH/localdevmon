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

    println!("{}", String::from_utf8_lossy(&buffer[..]));
}
