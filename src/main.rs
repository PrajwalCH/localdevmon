use std::process::exit;

use localdevmon::{
    args_parser::{parse_args, ArgsParserErr},
    server::{self, ServerConfig},
};

fn main() {
    let mut server_config = ServerConfig::default();

    let path = parse_args();

    match path {
        Ok(provided_path) => server_config.path = provided_path,
        Err(err) => {
            if err == ArgsParserErr::PathIsNotValid {
                println!("Given path is not valid");
                exit(1);
            }
        }
    }

    server::start(server_config);
}
