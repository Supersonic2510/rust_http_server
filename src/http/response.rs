use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use super::status::HTTPStatus;
use super::version::HTTPVersion;

#[derive(Clone)]
pub struct HTTPResponse {
    version: HTTPVersion,
    status_code: u16,
    status_message: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HTTPResponse {
    pub fn new(version: HTTPVersion, status: HTTPStatus, headers: HashMap<String, String>, body: Option<Vec<u8>>) -> Self {

        HTTPResponse {
            version,
            status_code: status.code(),
            status_message: status.message().to_owned(),
            headers,
            body,
        }
    }
}

impl Display for HTTPResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // write the version, status code, and status message
        write!(f, "{} {} {}\r\n", self.version, self.status_code, self.status_message)?;

        // loop over each header and write it to the formatter.
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }

        // write the body if it exists, else write newline.
        if let Some(body) = &self.body {
            write!(f, "\r\n{}", String::from_utf8_lossy(body))?;
        } else {
            write!(f, "\r\n")?;
        }

        Ok(())
    }
}