///
/// # Type
/// Request : request type to be built to send to the web server
///
pub struct Request {
    //
    // Start-line holds:
    // - METHOD (GET, POST, HEAD, OPTIONS...)
    // - URL path (/foo.html, ...)
    // - PROTO VERSION (HTTP/1.0, HTTP/1.1, HTTP/2, HTTP/3)
    // - CRLF '\r\n'
    //
    pub start_line: String, // "GET /foo.html HTTP/1.0 \r\n"

    //
    // Headers holds:
    // - Host (www.example.com)
    // - Key-Value pair (Accept: */*)
    // - CRLF '\r\n\r\n'
    //
    pub headers: Vec<String>,

    //
    // Body holds: # for now
    // - JSON DATA ( { "key": "value"} )
    // - form submission (name=John+Doe&email=example%40gmail.com)
    pub body: Vec<String>,
}

impl Request {
    pub fn new(start_line: &str) -> Self {
        Self {
            start_line: start_line.to_owned(),
            headers: vec!["\r\n".to_owned()],
            body: vec![],
        }
    }
}

///
/// Implement Builder Design pattern to create the request.
///
/// # Example
/// ```
/// use crate::request::*;
/// fn main() {
///    let req = Request::new("GET /FUZZ HTTP/1.0\r\n");
///    req = req
///           .push_header("Content-Type: text/html")
///           .push_body("user_id=1");
/// }
/// ```
///
impl Request {
    // the program expects the http method (DEFAULT: GET) and URL value
    pub fn push_start_line(mut self, start_line: &str) -> Self {
        self.start_line = start_line.to_owned();
        self
    }

    // (optional) the program expects key-value pair (default: "").
    pub fn push_header(mut self, header: &str) -> Self {
        self.headers.push(header.to_owned());
        self
    }

    // (optional) the program expects a payload either json or form submission data.
    pub fn push_body(mut self, payload: &str) -> Self {
        self.body.push(payload.to_owned());
        self
    }
}
