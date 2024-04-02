use std::{env, fmt};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::status::HTTPStatus;
use super::method::HTTPMethod;
use super::response::HTTPResponse;
use super::version::HTTPVersion;

const CONTENT_TYPE: &str = "Content-Type";
const CONTENT_LENGTH: &str = "Content-Length";
const USER_AGENT: &str = "User-Agent";
const TEXT_PLAIN: &str = "text/plain";

#[derive(Clone)]
pub struct HTTPRequest {
    method: HTTPMethod,
    path: PathBuf,
    version: HTTPVersion,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HTTPRequest {
    pub fn new(method: String, path: PathBuf, version: String, headers: HashMap<String, String>, body: Option<Vec<u8>>) -> Self {
        let method = HTTPMethod::from_str(&method).unwrap_or(HTTPMethod::GET);
        let version = HTTPVersion::from_str(&version).unwrap_or(HTTPVersion::HTTP1_0);

        HTTPRequest {
            method,
            path,
            version,
            headers,
            body,
        }
    }

    pub fn route_request(&self, path_dir_str: Option<String>) -> Result<HTTPResponse, Box<dyn Error>> {
        let path_str = self.path
            .to_str()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?;

        match path_str {
            "/" if self.method == HTTPMethod::GET => self.handle_root_request(&path_dir_str),
            _ if path_str.starts_with("/echo/") && self.method == HTTPMethod::GET => self.handle_echo_request(path_str),
            _ if path_str.starts_with("/user-agent") && self.method == HTTPMethod::GET =>  self.handle_user_agent_request(),
            _ if self.method == HTTPMethod::GET => self.handle_file_get_request(&path_dir_str, path_str),
            _ if self.method == HTTPMethod::POST => self.handle_file_post_request(&path_dir_str, path_str),
            _ => Ok(HTTPResponse::new(HTTPVersion::HTTP1_1, HTTPStatus::NotFound, HashMap::new(), None)),
        }
    }

    fn handle_root_request(&self, dir: &Option<String>) -> Result<HTTPResponse, Box<dyn Error>> {
        let dir_to_use = self.get_directory_to_use(dir)?;
        let full_file_path = format!("{}/{}", dir_to_use, "index.html");
        let extension = "html";

        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE.to_owned(), self.get_content_type(extension));

        let content = std::fs::read_to_string(&full_file_path).unwrap_or_else(|_| String::from("Error: Cannot read file"));
        headers.insert(CONTENT_LENGTH.to_owned(), content.len().to_string());

        let body: Vec<u8> = content.as_bytes().to_vec();
        Ok(HTTPResponse::new(HTTPVersion::HTTP1_1, HTTPStatus::OK, headers, Some(body)))
    }

    fn handle_echo_request(&self, path_str: &str) -> Result<HTTPResponse, Box<dyn Error>> {
        let echo = path_str.trim_start_matches("/echo/");
        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE.to_owned(), TEXT_PLAIN.into());
        headers.insert(CONTENT_LENGTH.into(), echo.len().to_string());

        let body: Vec<u8> = echo.as_bytes().to_vec();
        Ok(HTTPResponse::new(HTTPVersion::HTTP1_1, HTTPStatus::OK, headers, Some(body)))
    }

    fn handle_user_agent_request(&self) -> Result<HTTPResponse, Box<dyn Error>> {
        let user_agent_value = match self.headers.get(USER_AGENT) {
            Some(agent) => agent.clone(),
            None => "Unable to determine User-Agent".to_string(),
        };

        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE.to_owned(), TEXT_PLAIN.into());
        headers.insert(CONTENT_LENGTH.into(), user_agent_value.len().to_string());

        let body: Vec<u8> = user_agent_value.as_bytes().to_vec();

        Ok(HTTPResponse::new(HTTPVersion::HTTP1_1, HTTPStatus::OK, headers, Some(body)))
    }

    fn handle_file_get_request(&self, dir: &Option<String>, file_path: &str) -> Result<HTTPResponse, Box<dyn Error>> {

        let dir_to_use = self.get_directory_to_use(dir)?;
        let full_file_path = format!("{}/{}", dir_to_use, file_path);
        let extension = Path::new(&full_file_path).extension().and_then(|ext| ext.to_str()).unwrap_or("");

        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE.to_owned(), self.get_content_type(extension));

        let content = std::fs::read_to_string(&full_file_path)?;
        headers.insert(CONTENT_LENGTH.to_owned(), content.len().to_string());

        let body: Vec<u8> = content.as_bytes().to_vec();
        Ok(HTTPResponse::new(HTTPVersion::HTTP1_1, HTTPStatus::OK, headers, Some(body)))
    }

    fn handle_file_post_request(&self, dir: &Option<String>, file_path: &str) -> Result<HTTPResponse, Box<dyn Error>> {
        let dir_to_use = self.get_directory_to_use(dir)?;
        let full_file_path = format!("{}/{}", dir_to_use, file_path);
        let extension = Path::new(&full_file_path).extension().and_then(|ext| ext.to_str()).unwrap_or("");

        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE.to_string(), self.get_content_type(extension));

        let body_string = String::from_utf8(self.body.clone().unwrap_or_default()).unwrap_or_default();
        std::fs::write(&full_file_path, body_string.clone())?;

        headers.insert(CONTENT_LENGTH.to_owned(), body_string.len().to_string());
        Ok(HTTPResponse::new(HTTPVersion::HTTP1_1, HTTPStatus::Created, headers, None))
    }

    fn get_directory_to_use(&self, dir: &Option<String>) -> Result<String, std::io::Error> {
        match dir {
            Some(dir) => Ok(dir.clone()),
            None => env::current_dir()?.to_str().map(|s| s.to_string()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))
        }
    }

    // Helper function to get content type based on file extension
    fn get_content_type(&self, extension: &str) -> String {
        if matches!(extension, "html" | "css" | "js" | "png" | "jpg" | "jpeg" | "svg") {
            format!("text/{}", extension)
        } else {
            "application/octet-stream".to_string()
        }
    }

    pub fn keep_alive(&self) -> bool {
        match self.headers.get("Connection") {
            Some(conn) => !(conn.to_lowercase() == "close"),
            None => match self.version {
                HTTPVersion::HTTP1_0 => false, // expect connection to close by default for HTTP/1.0
                _ => true, // expect connection to keep-alive by default for HTTP/1.1 and HTTP/2
            }
        }
    }
}

impl Display for HTTPRequest {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // write the method, url, and version
        write!(f, "{} {} {}\r\n", self.method, self.path.to_str().unwrap_or(""), self.version)?;

        // loop over each header and write it to the formatter
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }

        // write the body if it exists
        if let Some(body) = &self.body {
            write!(f, "\r\n{}", String::from_utf8_lossy(body))?;
        } else {
            write!(f, "\r\n")?;
        }

        Ok(())
    }
}