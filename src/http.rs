use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;

use request::HTTPRequest;
use response::HTTPResponse;

mod request;
mod response;
mod method;
pub mod version;
pub mod status;

pub const EMPTY_LINE: &str = "\r\n";

pub struct HTTPReader{
    reader: BufReader<TcpStream>,
    http_request: Option<HTTPRequest>
}

impl HTTPReader {
    pub fn new(stream: &TcpStream) -> Self {
        let reader = BufReader::new(stream.try_clone().unwrap());
        HTTPReader {
            reader,
            http_request: None,
        }
    }

    pub fn read_request(&mut self) -> Result<(), String> {
        let mut raw_request = String::new(); // read the raw request from reader

        loop {
            let mut line_buffer: String = String::new();
            if let Err(e) = self.reader.read_line(&mut line_buffer) {
                let error = format!("Error responding status: {}", e);
                return Err(error);
            }

            if line_buffer == EMPTY_LINE {
                // End of HTTP headers section
                break;
            }

            raw_request.push_str(&line_buffer);
        }

        let (method_str, path, version_str, headers_map) = Self::parse_from_str(raw_request);

        let mut body : Option<Vec<u8>> = None;
        if let Some(content_length_str) = headers_map.get("Content-Length") {
            if let Ok(content_length) = content_length_str.parse::<usize>() {
                if content_length > 0 { // there's a body
                    let mut body_vec = vec![0u8; content_length];
                    self.reader.read_exact(&mut body_vec).unwrap();
                    body = Some(String::from_utf8(body_vec).unwrap().into_bytes());
                }
            }
        }

        // Create a new HTTPRequest instance
        let http_request = HTTPRequest::new(method_str, PathBuf::from(path), version_str, headers_map, body);

        self.http_request = Some(http_request);

        return Ok(());
    }

    fn parse_from_str(raw_header: String) -> (String, String, String, HashMap<String, String>) {
        let mut lines = raw_header.split(EMPTY_LINE);
        let (method, uri, version) = Self::parse_request_line(lines.next().unwrap());
        let headers = Self::parse_headers(lines);

        (method, uri, version, headers)
    }

    fn parse_request_line(line: &str)  -> (String, String, String) {
        let mut parts = line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let uri = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();
        (method, uri, version)
    }

    fn parse_headers<'a, L>(mut lines: L) -> HashMap<String, String>
        where
            L: Iterator<Item = &'a str>,
    {
        let mut headers = HashMap::new();

        for line in &mut lines {
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap_or("").trim().to_string();
            let value = parts.next().unwrap_or("").trim().to_string();
            if !name.is_empty() && !value.is_empty() {
                headers.insert(name, value);
            }
        }
        headers
    }

    pub fn route_request(&self, path_str: Option<String>) -> Result<HTTPResponse, Box<dyn Error>> {
        match &self.http_request {
            Some(request) => request.route_request(path_str).map_err(|e| e.into()),
            None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "HTTP request not found"))),
        }
    }
    pub fn is_kept_alive(&self) -> bool {
        match &self.http_request {
            Some(request) => request.keep_alive(),
            None => false,
        }
    }
}

impl Display for HTTPReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.http_request {
            None => write!(f, "HTTPRequest: Empty"),
            Some(request) => write!(f, "HTTPRequest: {}", request),
        }
    }
}

pub struct HTTPWriter {
    writer: BufWriter<TcpStream>,
    http_response: Option<HTTPResponse>
}

impl HTTPWriter {
    pub fn new(stream: &TcpStream) -> Self {
        let writer = BufWriter::new(stream.try_clone().unwrap());
        HTTPWriter {
            writer,
            http_response: None,
        }
    }
    
    pub fn set_response(&mut self, http_response: HTTPResponse) {
        self.http_response = Some(http_response);
    }

    pub fn write_response(&mut self) -> Result<usize, Box<dyn Error>> {
        match &self.http_response {
            None => Err("Cannot write a empty response".into()),
            Some(response) => self.writer.write(response.to_string().as_bytes()).map_err(|e| e.into()),
        }
    }
}

impl Drop for HTTPWriter {
    fn drop(&mut self) {
        if let Err(e) = self.writer.flush() {
            eprintln!("Error occurred while flushing BufWriter: {}", e);
        }
    }
}
