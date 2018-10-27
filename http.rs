use chrono::Utc;
use std::collections::HashMap;
use std::io::{BufReader, Write, prelude::*};
use std::net;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    Extension(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum StatusCode {
    Continue,
    SwitchingProtocols,
    Processing,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
    Unregistered(u16),
}

#[derive(Debug, Clone)]
pub struct Request {
    pub header: RequestHeader,
    pub body: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct Response {
    pub header: ResponseHeader,
    //pub header: String,
    pub body: Vec<u8>,
    pub checksum: [u8; 32],
    sent: bool,
}

#[derive(Debug, Clone)]
pub struct Cookie {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct RequestHeader {
    pub method: Method,
    pub cookie: String,
    pub uri: String,
    pub version: String,
    pub Headers: HashMap<String, String>,
    pub Cookies: HashMap<String, String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ResponseHeader {
    pub version: String,
    pub code: u16,
    pub status: String,
    pub Headers: HashMap<String, String>,
    pub Cookies: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    HTTP,
    //HTTPS
}

#[derive(Debug, Clone)]
pub struct HTTP {
    pub address: String,
    pub port: u16,
    pub protocol: Protocol,
    pub request: Request,
    pub response: Response,
}

impl HTTP {
    pub fn new(
        address: String,
        port: u16,
        protocol: Protocol,
        method: Method,
        uri: String,
    ) -> HTTP {
        let mut http = HTTP {
            address: address.to_string(),
            port: port,
            protocol: protocol,
            request: Request {
                header: RequestHeader {
                    method: method,
                    cookie: String::from(""),
                    uri: uri,
                    version: String::from("HTTP/1.0"),
                    Headers: HashMap::new(),
                    Cookies: HashMap::new(),
                },
                body: Vec::new()
                //body: "".into_string(),
            },
            response: Response {
                header: ResponseHeader {
                    version: String::from(""),
                    code: 0,
                    status: String::from(""),
                    Headers: HashMap::new(),
                    Cookies: HashMap::new(),
                },
                //                header: String::from(""),
                body: Vec::new(),
                checksum: [0; 32],
                sent: false,
            },
        };
        http
            .request
            .add_header("Host".to_string(), address.to_string());
        http
    }
    pub fn set_body(&mut self, data: String) {
        self.request.body = data.to_string().into_bytes();
    }

    #[allow(dead_code)]
    pub fn set_cookie(&mut self, data: String) {
        self.request.header.cookie = data; // temp
    }

    //If is_sent returns false call do_request
    pub fn is_sent(&self) -> bool {
        self.response.sent
    }

    #[allow(dead_code)]
    pub fn new_from_str() -> HTTP {
        HTTP::new(
            "www.um.ac.ir".to_string(),
            80,
            Protocol::HTTP,
            Method::Get,
            "/".to_string(),
        )
    }

    #[allow(dead_code)]
    pub fn get_request(&mut self) -> &mut Request {
        &mut self.request
    }

    pub fn get_response(&mut self) -> &mut Response {
        &mut self.response
    }

    #[allow(dead_code)]
    pub fn fill_response(&mut self, _code: StatusCode) {
        self.response.header.version = "HTTP/1.0".to_string();
        self.response.header.code = 200;
        self.response.header.status = "OK".to_string();
        self.response.add_header(
            "Date".to_string(),
            Utc::now().format("%a, %e %b %Y %T GMT").to_string(),
        );
        let body_len = self.response.body.len();
        if body_len > 0 {
            self.response
                .add_header("Content-Length".to_string(), body_len.to_string());
        }
        self.response
            .add_header("Connection".to_string(), "close".to_string());
    }

    pub fn do_request(&mut self) {
        let mut stream =
            net::TcpStream::connect(format!("{}:{}", self.address, self.port).to_string()).unwrap();
        stream
            .write(
                format!(
                    "{} {} {}\r\n",
                    match self.request.header.method {
                        Method::Options => "OPTIONS",
                        Method::Post => "POST",
                        Method::Get => "GET",
                        Method::Put => "PUT",
                        Method::Delete => "DELETE",
                        Method::Head => "HEAD",
                        Method::Trace => "TRACE",
                        Method::Connect => "CONNECT",
                        Method::Patch => "PATCH",
                        _ => "GET",
                    },
                    self.request.header.uri,
                    self.request.header.version
                ).as_bytes(),
            )
            .unwrap();
        //            self.request.add_header(name, value)
        if self.request.header.Cookies.is_empty() == false {
            let mut cookie = "".to_string();
            for (key, value) in &self.request.header.Cookies {
                cookie.push_str(&format!("{}={};", key, value));
            }
            self.request.add_header("Cookie".to_string(), cookie);
        }
        if self.request.header.Headers.is_empty() == false {
            for (key, value) in &self.request.header.Headers {
                stream
                    .write(format!("{}: {}\r\n", key, value).as_bytes())
                    .unwrap();
            }
        }
        stream.write("\r\n".as_bytes()).unwrap();
        let mut response: String = "".to_string();
        BufReader::new(stream).lines().for_each(|x| {
            response.push_str(&format!(
                "{}\r\n",
                x.unwrap_or_else(move |_x| "BIN".to_string()),
            ))
        });
        let mut response = response.splitn(2, "\r\n\r\n");
        for (i, row) in response
            .next()
            .unwrap()
            .to_string()
            .split("\r\n")
            .enumerate() {
                if i != 0 {
                    let mut tmp = row.splitn(2, ": ");
                    self.response.header.Headers.insert(
                        tmp.next().unwrap().to_string(),
                        tmp.next().unwrap().to_string(),
                    );
                } else {
                    let mut tmp = row.splitn(3, " ");
                    self.response.header.version = tmp.next().unwrap().to_string();
                    self.response.header.code = tmp.next().unwrap().parse::<u16>().unwrap();
                    self.response.header.status = tmp.next().unwrap().to_string();
                }
        }
        self.response.body = response.next().unwrap().to_string().into_bytes();
    }
}

#[allow(dead_code)]
pub fn resolve_method(method: String) -> Method {
    let z = match method.to_ascii_uppercase().as_ref() {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        &_ => Method::Get,
    };
    z
}

#[allow(dead_code)]
impl Request {
    pub fn add_header(&mut self, name: String, value: String) {
        self.header.Headers.insert(name, value);
    }
    pub fn set_cookie(&mut self, name: String, value: String) {
        self.header.Cookies.insert(name, value);
    }
    pub fn body(&self) -> Vec<u8> {
        self.body.clone()
    }
    pub fn into_bytes(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut format!(
            "{} {} {}\r\n",
            match self.header.method {
                Method::Options => "OPTIONS",
                Method::Post => "POST",
                Method::Get => "GET",
                Method::Put => "PUT",
                Method::Delete => "DELETE",
                Method::Head => "HEAD",
                Method::Trace => "TRACE",
                Method::Connect => "CONNECT",
                Method::Patch => "PATCH",
                _ => "GET",
            },
            self.header.uri,
            self.header.version
        ).into_bytes());
        for (key, value) in &self.header.Headers {
            result.append(&mut format!("{}: {}\r\n", key, value).into_bytes());
        }
        result.append(&mut "\r\n".to_string().into_bytes());
        result.append(&mut self.body);
        result
    }
}

#[allow(dead_code)]
impl Response {
    pub fn add_header(&mut self, name: String, value: String) {
        self.header.Headers.insert(name, value);
    }
    pub fn set_cookie(&mut self, name: String, value: String) {
        self.header.Cookies.insert(name, value);
    }
    pub fn body(&self) -> Vec<u8> {
        self.body.clone()
    }
    pub fn into_bytes(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.append(&mut format!(
            "{} {} {}\r\n",
            self.header.version, self.header.code, self.header.status
        ).into_bytes());
        for (key, value) in &self.header.Headers {
            result.append(&mut format!("{}: {}\r\n", key, value).into_bytes());
        }
        result.append(&mut "\r\n".to_string().into_bytes());
        result.append(&mut self.body);
        result
    }
}
