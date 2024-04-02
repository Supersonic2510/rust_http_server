use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub enum HTTPVersion {
    HTTP1_0,
    HTTP1_1,
    HTTP2_0,
    // ...
}

impl FromStr for HTTPVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HTTPVersion::HTTP1_0),
            "HTTP/1.1" => Ok(HTTPVersion::HTTP1_1),
            "HTTP/2.0" => Ok(HTTPVersion::HTTP2_0),
            _ => Err(()),
        }
    }
}

impl Display for HTTPVersion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let version = match self {
            HTTPVersion::HTTP1_0 => "HTTP/1.0",
            HTTPVersion::HTTP1_1 => "HTTP/1.1",
            HTTPVersion::HTTP2_0 => "HTTP/2.0",
            // ...
        };
        write!(f, "{}", version)
    }
}