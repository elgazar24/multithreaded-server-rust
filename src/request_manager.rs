use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::{BufRead, BufReader, Write};
use std::{fs, net::TcpStream, path::Path};

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    _version: String,
}

#[derive(Debug)]
struct HttpResponse {
    status_line: String,
    content_type: String,
    content: String,
}

pub struct RequestManager {}

impl RequestManager {
    // fn new() -> Self {
    //     RequestManager {}
    // }

    pub fn handle_request(mut stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        // Parse the HTTP request
        let request = RequestManager::parse_request(&mut stream)?;

        // Generate the appropriate response
        let response = RequestManager::generate_response(&request)?;

        // Send the response
        RequestManager::send_response(&mut stream, &response)?;

        Ok(())
    }

    fn parse_request(stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn std::error::Error>> {
        let buf_reader = BufReader::new(stream);
        let request_line = buf_reader
            .lines()
            .next()
            .ok_or("No request line received")??;

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            // Normal message format not http format will echo the message back

            return  Ok(HttpRequest {
                method: String::from("MESSAGE"),
                path: (parts[0].to_string() + " " + parts[1] + " " + parts[2] ).to_string(),
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
            ("MESSAGE", path )  =>  RequestManager::serve_message(path),
            ("INVALID", path )   => RequestManager::serve_400(),
            _ => RequestManager::serve_404(),
        }
    }
    fn serve_message(message: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        Ok(HttpResponse {
            status_line: "MESSAGE".to_string(),
            content_type: "MESSAGE".to_string(),
            content: message.to_string(),
        })
    }
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
    // fn serve_pdf(filepath: &str, content_type: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    //     if !Path::new(filepath).exists() {
    //         return ServerManager::serve_404();
    //     }

    //     // Read the file as binary data
    //     let binary_content = fs::read(filepath)?;

    //     // Encode the binary data as a Base64 string
    //     let content = STANDARD.encode(binary_content);

    //     Ok(HttpResponse {
    //         status_line: "HTTP/1.1 200 OK".to_string(),
    //         content_type: content_type.to_string(),
    //         content, // Base64 encoded string
    //     })
    // }

    fn serve_404() -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("static/404.html")
            .unwrap_or_else(|_| "<h1>404 - Page Not Found</h1>".to_string());

        Ok(HttpResponse {
            status_line: "HTTP/1.1 404 NOT FOUND".to_string(),
            content_type: "text/html".to_string(),
            content,
        })
    }
    fn serve_400() -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("static/400.html")
            .unwrap_or_else(|_| "<h1>400 - Bad Request</h1>".to_string());

        Ok(HttpResponse {
            status_line: "HTTP/1.1 400 Bad Request".to_string(),
            content_type: "text/html".to_string(),
            content,
        })
    }

    fn send_response(
        stream: &mut TcpStream,
        response: &HttpResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {

        let response_string : String;

        if response.status_line == "MESSAGE" {
            response_string = response.content.clone();
        }else {
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
