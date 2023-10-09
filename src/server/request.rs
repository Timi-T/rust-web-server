//! # use server::request::{ [RequestObject], [handle_connection] }
//! 
//! A library that hanldes requests made to the web-server.

use std::{fs, net::TcpStream};
use std::io::prelude::*;
use super::process::CliArgs;

#[derive(Debug)]
/// The requests made to the web-server are represented in the data structure `RequestObject`
/// The only supported request method on the server is `GET`
pub struct RequestObject {
    /// This represents the request method of the request received by the server
    pub method: String,

    /// This represents the route of the asset/file requested by the client side.
    pub route: String,
}

impl RequestObject {
    /// Returns a struct with a request method and request route.
    /// 
    /// # Args
    /// 
    /// * `request` - This is the request in a string format containing necessary request information.
    /// 
    /// # Example
    /// 
    /// ```
    /// use server::request::RequestObject;
    /// let req_obj: Option<RequestObject> = RequestObject::new(&String::from("GET / Content-Length: 0\r\n\r\n"));
    /// 
    /// match req_obj {
    ///     Some(obj) => {
    ///         assert!(obj.method == "GET");
    ///         assert!(obj.route == "/");
    ///     }
    ///     None => {}
    /// }
    /// 
    /// ```
    /// 
    /// # Panics
    /// 
    /// - Panics if the request methos is not a `GET` method
    /// - Panics if the route does not start with `/`.
    ///  
    pub fn new(request: &String) -> Option<RequestObject> {
        let allowed_methods = ["GET"];
        let req_args: Vec<&str> = request.split(" ").collect();
        let method = req_args[0];
        let route = req_args[1];

        assert!(allowed_methods.contains(&method), "Invalid request method");
        assert!(route.starts_with("/"), "Invalid request route");

        Some(RequestObject {
            method: String::from(method),
            route: String::from(route)
        })
    }   
}

/// Function that handles a request stream.
/// The request is processed based on the method and route.
/// The route determines the file path for a `GET` request.
/// 
/// Args
/// 
/// * `stream` - A TcpStream type which basically is a stream of bytes that represents the client request.
/// 
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let str_buffer = String::from_utf8(buffer.clone().to_vec()).unwrap();
    let request_obj = RequestObject::new(&str_buffer);

    match &request_obj {
        Some(req) => {
            if req.method == "GET" {
                let route = req.route.as_str();
                let content: String;
                match route {
                    value => {
                        let mut file_path = value;
                        if (!value.contains(".") && !value.contains(".html")) || value == "/index.html" {
                            file_path = "/index.html"
                        }
                        let file_content = fs::read_to_string(String::from(CliArgs::get().dir_path) + file_path);

                        match file_content {
                            Ok(data) => {
                                content = data.clone()
                            }
                            Err(_) => {
                                content = fs::read_to_string("assets/404.html").unwrap();
                            }
                        }
                    }
                }
                
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content,
                );
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
        None => {}
    }
}
