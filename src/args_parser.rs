use std::env;
use std::path::PathBuf;

#[derive(Eq, PartialEq)]
pub enum ArgsParserErr {
    PathNotProvided,
    PathIsNotValid
}

pub fn parse_args() -> Result<PathBuf, ArgsParserErr> {
    let argv: Vec<String> = env::args().collect();

    if argv.len() == 1 {
        return Err(ArgsParserErr::PathNotProvided);
    }

    let path = PathBuf::from(&argv[1]);

    if !path.is_dir() {
        return Err(ArgsParserErr::PathIsNotValid);
    }

    Ok(path)
}
