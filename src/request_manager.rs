use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::Write;
use std::{fs, net::TcpStream, path::Path};

use std::io::Read;

// HttpRequest struct
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    _version: String,
}

// HttpResponse struct
#[derive(Debug)]
struct HttpResponse {
    status_line: String,
    content_type: String,
    content: String,
}

/// RequestManager
/// Handles HTTP requests
///     - Parses the request
///     - Generates the response
///     - Sends the response
pub struct RequestManager {}

impl RequestManager {
    /// Optional constructor
    // fn new() -> Self {
    //     RequestManager {}
    // }

    pub fn handle_request(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        // Parse the HTTP request
        let request = RequestManager::parse_request(stream)?;

        // Generate the appropriate response
        let response = RequestManager::generate_response(&request)?;

        // Send the response
        RequestManager::send_response(stream, &response)?;

        Ok(())
    }

    fn parse_request(stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn std::error::Error>> {
        // read the request
        let mut buffer = [0; 512];
        // Read data from the client
        let bytes_read = stream.read(&mut buffer)?;

        let request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        // Split the request line into method, path, and version and remove \nHost:
        let mut parts: Vec<&str> = request.split(" ").collect();

        if parts.len() >= 3 {
            parts[2] = parts[2].split("\r\n").collect::<Vec<&str>>()[0];
        }

        if parts[0] != "GET"
            && parts[0] != "POST"
            && parts[0] != "PUT"
            && parts[0] != "DELETE"
            && parts[0] != "INVALID"
        {
            // Normal message format not http format will echo the message back
            return Ok(HttpRequest {
                method: String::from("MESSAGE"),
                path: request,
                _version: String::from("MESSAGE"),
            });
            // return Err("Invalid HTTP request format , message sent back".into());
        }

        Ok(HttpRequest {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            _version: parts[2].to_string(),
        })
    }

    /// Generates the appropriate response
    fn generate_response(
        request: &HttpRequest,
    ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        match (request.method.as_str(), request.path.as_str()) {
            ("GET", "/") => RequestManager::serve_file("static/index.html", "text/html"),
            ("GET", path) if path.ends_with(".html") => {
                RequestManager::serve_file(&format!("static{}", path), "text/html")
            }
            ("GET", path) if path.ends_with(".css") => {
                RequestManager::serve_file(&format!("static{}", path), "text/css")
            }
            ("GET", path) if path.ends_with(".js") => {
                RequestManager::serve_file(&format!("static{}", path), "application/javascript")
            }
            // ("GET", path) if path.ends_with(".pdf") => ServerManager::serve_pdf(&format!("static{}", path), "application/pdf"),
            ("GET", path) if path.ends_with(".png") => {
                RequestManager::serve_image(&format!("static{}", path), "image/png")
            }
            ("GET", path) if path.ends_with(".jpg") => {
                RequestManager::serve_image(&format!("static{}", path), "image/jpeg")
            }
            ("MESSAGE", path) => RequestManager::serve_message(path),
            ("INVALID", _) => RequestManager::serve_400(),
            _ => RequestManager::serve_404(),
        }
    }

    /// Serves a row messages
    fn serve_message(message: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        Ok(HttpResponse {
            status_line: "MESSAGE".to_string(),
            content_type: "MESSAGE".to_string(),
            content: message.to_string(),
        })
    }
    /// Serves an image
    fn serve_image(
        filepath: &str,
        content_type: &str,
    ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        if !Path::new(filepath).exists() {
            return RequestManager::serve_404();
        }

        let content = fs::read(filepath)?;

        let content = STANDARD.encode(content);

        Ok(HttpResponse {
            status_line: "HTTP/1.1 200 OK".to_string(),
            content_type: content_type.to_string(),
            content,
        })
    }

    /// Serves a file
    fn serve_file(
        filepath: &str,
        content_type: &str,
    ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        if !Path::new(filepath).exists() {
            return RequestManager::serve_404();
        }

        let content = fs::read_to_string(filepath)?;

        Ok(HttpResponse {
            status_line: "HTTP/1.1 200 OK".to_string(),
            content_type: content_type.to_string(),
            content,
        })
    }

    /// Serves a 404 Not Found page
    fn serve_404() -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("static/404.html")
            .unwrap_or_else(|_| "<h1>404 - Page Not Found</h1>".to_string());

        Ok(HttpResponse {
            status_line: "HTTP/1.1 404 NOT FOUND".to_string(),
            content_type: "text/html".to_string(),
            content,
        })
    }

    /// Serves a 400 Bad Request
    fn serve_400() -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("static/400.html")
            .unwrap_or_else(|_| "<h1>400 - Bad Request</h1>".to_string());

        Ok(HttpResponse {
            status_line: "HTTP/1.1 400 Bad Request".to_string(),
            content_type: "text/html".to_string(),
            content,
        })
    }

    /// Sends the response
    fn send_response(
        stream: &mut TcpStream,
        response: &HttpResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response_string: String;

        if response.status_line == "MESSAGE" {
            response_string = response.content.clone();
        } else {
            response_string = format!(
                "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                response.status_line,
                response.content_type,
                response.content.len(),
                response.content
            );
        }

        stream.write_all(response_string.as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}
