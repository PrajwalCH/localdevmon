mod connection_handler;
mod route_handler;

use std::env;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::path::PathBuf;

use connection_handler::ConnectionHandler;
use route_handler::gen_dir_tree;

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
            path: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
}

pub fn start(server_config: ServerConfig) -> io::Result<()> {
    let dir_root_node = gen_dir_tree(&server_config.path)?;
    let listener = tcp_listen(server_config.host_addr, server_config.port_num)?;

    for stream in listener.incoming() {
        let stream = stream?;
        let _conn_handler = ConnectionHandler::new(stream, &dir_root_node);
    }

    Ok(())
}

fn tcp_listen(host_addr: Ipv4Addr, port_num: u16) -> io::Result<TcpListener> {
    let sock_addr = SocketAddr::from((host_addr, port_num));
    let listener = TcpListener::bind(sock_addr)?;

    println!("Server listening on: http://localhost:{}", port_num);

    Ok(listener)
}
