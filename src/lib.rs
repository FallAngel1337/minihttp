//! smolhttp is a simple and lightweight http client lib for rust.
//! only provide basic http client feature, more like python request.
//! smolhttp's aim is simple, easy to use, less dependent, smaller binary.
//! Enjoy yourself...
//!
//! # Example
//! ## Sending a GET request
//! ```no_run
//! // Using the shortcut function
//! let content = smolhttp::get("https://www.rust-lang.org").unwrap().text();
//! println!("{content}");
//!
//! // Using the Client
//! let content = smolhttp::Client::new("https://www.rust-lang.org").unwrap().get().send().unwrap().text();
//! println!("{content}");
//! 
//! # Example
//! ## Sending a POST request
//! ```no_run
//! // Using the shortcut funtion
//! let content = smolhttp::post("https://www.rust-lang.org").unwrap().text();
//! println!("{content}");
//!
//! // Using the Client
//! let content = smolhttp::Client::new("https://www.rust-lang.org")
//!   .unwrap()
//!   .post()
//!   .send()
//!   .unwrap()
//!   .text();
//! println!("{content}");
//! ```
//!
//! ## Using custom headers
//! ```no_run
//! use std::collections::HashMap;
//! let content = smolhttp::Client::new("https://www.rust-lang.org")
//!   .unwrap()
//!   .post()
//!   .headers(vec![("User-Agent".to_owned(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_owned())])
//!   .send()
//!   .unwrap()
//!   .text();
//! println!("{content}");
//! ```
//!
//! ## Using a proxy
//! ```no_run
//! let content = smolhttp::Client::new("http://www.google.com")
//!   .unwrap()
//!   .proxy("http://127.0.0.1:1080")
//!   .unwrap()
//!   .get()
//!   .send()
//!   .unwrap()
//!   .text();
//! println!("{content}");
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
