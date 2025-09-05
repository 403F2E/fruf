///
/// # Type
/// Request : request type to be built to send to the web server
///
/// # Idea
/// in this file I m going to try to build the request myself and send it myself instead of "reqwest" crate
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

pub struct RequestBuilder {
    url: String,
    method: Option<String>,
    headers: Option<Vec<String>>,
    body: Option<Vec<String>>,
}

impl Request {
    pub fn new(url: &str, method: &str) -> RequestBuilder {
        RequestBuilder {
            method: Some(method.to_owned()),
            url: url.to_owned(),
            headers: Some(vec!["\r\n".to_owned()]),
            body: Some(vec![]),
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
impl RequestBuilder {
    // the program expects a URL
    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_owned();
        self
    }

    // the program expects a HTTP Method (Default: GET)
    pub fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_owned());
        self
    }

    // (optional) the program expects key-value pair (default: "").
    pub fn header(mut self, key: &str, value: &str) -> Self {
        if let Some(ref mut headers) = self.headers {
            let _ = headers.pop();
            headers.push(format!("{} : {}", key, value));
            headers.push("\r\n".to_owned());
        }
        self
    }

    // (optional) the program expects a payload either json or form submission data.
    pub fn body(mut self, payload: &str) -> Self {
        if let Some(ref mut body) = self.body {
            body.push(payload.to_owned());
        }
        self
    }

    // to continue working on
    pub fn build(self) -> Request {
        let _start_line: String = format!("{} {} HTTP/1.0 \r\n", self.method.unwrap(), self.url);
        let headers: Vec<String> = self.headers.unwrap();
        let body: Vec<String> = self.body.unwrap();
        Request {
            start_line: "".to_owned(),
            headers: headers,
            body: body,
        }
    }
}
