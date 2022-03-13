//! minihttp is a simple and lightweight http client lib for rust.
//! only provide basic http client feature, more like python request.
//! minihttp's aim is simple, easy to use, less dependent, smaller binary.
//! Enjoy yourself...
//!
//! #Example
//! ## send a get request
//! ```no_run
//! use minihttp::request::Request;
//!
//! let mut http = Request::new("http://www.google.com").unwrap();
//! let res = http.get().send().unwrap();
//! println!("status code {}",res.status_code());
//! println!("reason {}",res.reason());
//! println!("body {}",res.text());
//! for (k,v) in res.headers(){
//!     println!("{}:{}",k,v);
//! }
//! ```
//!
//! ## send a post request
//! ```no_run
//! use minihttp::request::Request;
//!
//! let mut http = Request::new("http://www.google.com").unwrap();
//! let res = http.post().body_str("hello").send().unwrap();
//! println!("status code {}",res.status_code());
//! ```
//!
//! ## custom headers
//! ```no_run
//! use std::collections::HashMap;
//! use minihttp::request::Request;
//!
//! let mut http = Request::new("http://www.google.com").unwrap();
//! let mut headers = HashMap::new();
//! headers.insert("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36");
//! let res = http.headers(headers).send().unwrap();
//! println!("status code {}",res.status_code());
//! ```
//!
//! ## support https
//! ```no_run
//! use minihttp::request::Request;
//!
//! let mut http = Request::new("https://www.google.com").unwrap();
//! let res = http.get().send().unwrap();
//! println!("status code {}",res.status_code());
//! ```
//!
//! ## support proxy
//! ```no_run
//! use minihttp::request::Request;
//!
//! let mut http = Request::new("https://www.google.com").unwrap();
//! let res = http.proxy("https://127.0.0.1:1080").unwrap().get().send().unwrap();
//! println!("status code {}",res.status_code());
//! ```
//!

#![doc(html_root_url = "https://docs.rs/minihttp")]

extern crate minihttpse;
extern crate miniurl;
extern crate native_tls;

use minihttpse::Response;
use miniurl::Url;
use native_tls::TlsConnector;
use native_tls::{Error, HandshakeError};

use std::fmt;
use std::io;
use std::net::TcpStream;

///http request module
mod request;
pub use request::{Client, Proxy};

///http basic error type
#[derive(Debug)]
pub enum HttpError {
    Parse(&'static str),
    Config(&'static str),
    Proxy(&'static str),
    IO(io::Error),
    SSL(Error),
    SSLHandshake(HandshakeError<TcpStream>),
}


///set Request GET method
/// # Example
/// ```
/// let content = minihttp::get("https://www.google.com").unwrap().text();
/// ```
#[inline(always)]
pub fn get(url: &str) -> Result<Response, HttpError> {
    request::Client::new(url)?.get().send()
}

///set Request POST method
/// # Example
/// ```
/// let content = minihttp::post("https://www.google.com").unwrap().text();
/// ```
#[inline(always)]
pub fn post(url: &str) -> Result<Response, HttpError> {
    request::Client::new(url)?.post().send()
}

///set Request HEAD method
/// # Example
/// ```
/// let content = minihttp::head("https://www.google.com").unwrap().text();
/// ```
#[inline(always)]
pub fn head(url: &str) -> Result<Response, HttpError> {
    request::Client::new(url)?.head().send()
}

///set Request DELETE method
/// # Example
/// ```
/// let content = minihttp::delete("https://www.google.com").unwrap().text();
/// ```
#[inline(always)]
pub fn delete(url: &str) -> Result<Response, HttpError> {
    request::Client::new(url)?.delete().send()
}

///set Request PUT method
/// # Example
/// ```
/// let content = minihttp::put("https://www.google.com").unwrap().text();
/// ```
#[inline(always)]
pub fn put(url: &str) -> Result<Response, HttpError> {
    request::Client::new(url)?.put().send()
}

///set Request OPTIONS method
/// # Example
/// ```
/// let content = minihttp::options("https://www.google.com").unwrap().text();
/// ```
#[inline(always)]
pub fn options(url: &str) -> Result<Response, HttpError> {
    request::Client::new(url)?.options().send()
}

impl std::error::Error for HttpError {}

impl From<io::Error> for HttpError {
    fn from(err: io::Error) -> HttpError {
        HttpError::IO(err)
    }
}

impl From<minihttpse::HttpError> for HttpError {
    fn from(err: minihttpse::HttpError) -> HttpError {
        let minihttpse::HttpError::Parse(e) = err;
        HttpError::Parse(e)
    }
}

impl From<HandshakeError<TcpStream>> for HttpError {
    fn from(err: HandshakeError<TcpStream>) -> HttpError {
        HttpError::SSLHandshake(err)
    }
}

impl From<Error> for HttpError {
    fn from(err: Error) -> HttpError {
        HttpError::SSL(err)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpError::Parse(ref err) => write!(f, "Parse error: {}", err),
            HttpError::Config(ref err) => write!(f, "Config error: {}", err),
            HttpError::IO(ref err) => write!(f, "IO error: {}", err),
            HttpError::Proxy(ref err) => write!(f, "Proxy error : {}", err),
            HttpError::SSL(ref err) => write!(f, "SSL error: {}", err),
            HttpError::SSLHandshake(ref err) => write!(f, "SSL handshake error: {}", err),
        }
    }
}
