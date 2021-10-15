mod map_rout;

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
    let sock_addr = SocketAddr::from((server_config.host_addr, server_config.port_num));
    let listener = TcpListener::bind(sock_addr)?;

    for stream in listener.incoming() {
        let stream = stream?;

        println!("Connection established");
        handle_request(stream);
    }

    println!("{:#?}", routes);
    println!("{:?}", sock_addr);
    Ok(())
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));
}
