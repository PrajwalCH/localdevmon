pub mod map_route;

use std::net::{Ipv4Addr};
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerConfig {
    pub port_num: u16,
    pub host_addr: Ipv4Addr,
    pub path: PathBuf
}

impl ServerConfig {
    pub fn init_default() -> ServerConfig {
        ServerConfig {
            port_num: 8000,
            host_addr: Ipv4Addr::new(127, 0, 0, 1),
            path: env::current_dir().unwrap_or(PathBuf::from("." ))
        }
    }
}
