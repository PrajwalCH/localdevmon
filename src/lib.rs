pub mod args_parser;
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_route_mapper() {
        use super::server::map_route::*;
        use std::env;
        use std::path::PathBuf;;
        let current_path = env::current_dir().unwrap_or(PathBuf::from("/"));
        let routes = map_route(&current_path);
        println!("2");

        for route in routes.iter() {
            println!("{:#?}", route);
        }
    }
}
