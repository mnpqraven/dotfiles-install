use std::{error::Error, fs::metadata};
use url::{ParseError, Url};

pub enum PathType {
    Dir,
    Url,
}

pub fn handle_path_type(path: &str) -> Result<PathType, Box<dyn Error>> {
    match true {
        true if metadata(path).is_ok() => Ok(PathType::Dir),
        true if Url::parse(path).is_ok() => Ok(PathType::Url),
        _ => Err(Box::new(ParseError::InvalidDomainCharacter)),
    }
}
