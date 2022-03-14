use super::HttpError;
use super::Response;
use super::TlsConnector;
use super::Url;

use std::net::TcpStream;
use std::time;

#[derive(Debug, Clone)]
pub enum Methods {
    Get,
    Post,
    Put,
    Head,
    Delete,
    Options,
    Custom(String),
}

///proxy info object.
#[derive(Debug, Clone)]
pub struct Proxy(Url);

///http request object.
#[derive(Debug, Clone)]
pub struct Client {
    host: String,
    port: u16,
    scheme: String,
    method: Methods,
    url: Url,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
    timeout: u64,
    proxy: Option<Proxy>,
    verify: bool,
}

impl std::fmt::Display for Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Methods::*;
        match self {
            Get => write!(f, "GET"),
            Post => write!(f, "POST"),
            Put => write!(f, "PUT"),
            Head => write!(f, "HEAD"),
            Delete => write!(f, "DELETE"),
            Options => write!(f, "OPTIONS"),
            Custom(method) => write!(f, "{method}"),
        }
    }
}

impl Client {
    ///return a Request object
    /// # Example
    /// ```
    /// use minihttp::Request;
    ///
    /// let mut http = Request::new("https://www.google.com").unwrap();
    /// ```
    pub fn new(url: &str) -> Result<Self, HttpError> {
        let url: Url = Url::parse(url);

        let host = match url.host {
            Some(ref h) => h.clone(),
            None => return Err(HttpError::Parse("url parse error")),
        };
        Ok(Self {
            host,
            port: url.port,
            scheme: url.scheme.clone(),
            method: Methods::Get,
            url,
            headers: Vec::new(),
            body: None,
            timeout: 30,
            proxy: None,
            verify: true,
        })
    }

    ///set Request GET method
    /// # Example
    /// ```
    /// use minihttp::CLient;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.get();
    /// ```
    pub fn get(&mut self) -> &mut Self {
        self.method = Methods::Get;
        self
    }

    ///set Request POST method
    /// # Example
    /// ```
    /// use minihttp::CLient;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.post();
    /// ```
    pub fn post(&mut self) -> &mut Self {
        self.method = Methods::Post;
        self
    }

    ///set Request PUT method
    /// # Example
    /// ```
    /// use minihttp::CLient;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.put();
    /// ```
    pub fn put(&mut self) -> &mut Self {
        self.method = Methods::Put;
        self
    }

    ///set Request HEAD method
    /// # Example
    /// ```
    /// use minihttp::CLient;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.head();
    /// ```
    pub fn head(&mut self) -> &mut Self {
        self.method = Methods::Head;
        self
    }

    ///set Request DELETE method
    /// # Example
    /// ```
    /// use minihttp::CLient;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.delete();
    /// ```
    pub fn delete(&mut self) -> &mut Self {
        self.method = Methods::Delete;
        self
    }

    ///set Request OPTION method
    /// # Example
    /// ```
    /// use minihttp::CLient;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.options();
    /// ```
    pub fn options(&mut self) -> &mut Self {
        self.method = Methods::Options;
        self
    }

    ///set Client's custom method
    /// # Example
    /// ```
    /// use minihttp::Client;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.request("profile");
    /// ```
    pub fn request(&mut self, method: &str) -> &mut Self {
        self.method = Methods::Custom(method.to_owned());
        self
    }

    ///set Client's custom header
    /// # Example
    /// ```
    /// use minihttp::Client;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// let mut headers = vec![("User-Agent".to_owned(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_owned())]
    /// client.headers(headers);
    /// ```
    pub fn headers(&mut self, data: Vec<(String, String)>) -> &mut Self {
        self.headers = data;
        self
    }

    ///set Client's body
    /// # Example
    /// ```
    /// use minihttp::Client;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// let body = vec![0,1,2,3,4];
    /// client.body(body);
    /// ```
    pub fn body(&mut self, data: Vec<u8>) -> &mut Self {
        self.body = Some(data);
        self
    }

    ///set Client's read/write timeout(sec)
    /// # Example
    /// ```
    /// use minihttp::Client;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.timeout(10);
    /// ```
    pub fn timeout(&mut self, time: u64) -> &mut Self {
        self.timeout = time;
        self
    }

    ///set http(s) request if verify the certificate(default true)
    /// # Example
    /// ```
    /// use minihttp::Client;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.verify(false);
    /// ```
    pub fn verify(&mut self, verify: bool) -> Result<&mut Self, HttpError> {
        if self.scheme == "https" {
            self.verify = verify;
        } else {
            return Err(HttpError::Config("Verify setting only for https"));
        }
        Ok(self)
    }

    ///set proxy info
    /// # Example
    /// ```
    /// use minihttp::Client;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.proxy("https://127.0.0.1:1080");
    /// ```
    pub fn proxy(&mut self, proxy: &str) -> Result<&mut Self, HttpError> {
        let url: Url = Url::parse(proxy);

        if self.scheme == "https" && url.scheme == "http" {
            return Err(HttpError::Proxy("Http proxy can only use http scheme."));
        }

        self.proxy = Some(Proxy(url));
        Ok(self)
    }

    ///send http(s) request
    /// # Example
    /// ```
    /// use minihttp::Client;
    ///
    /// let mut client = Client::new("https://www.google.com").unwrap();
    /// client.request("GET").send();
    /// ```
    pub fn send(&mut self) -> Result<Response, HttpError> {
        let header = self.build_header();
        let connector = TlsConnector::builder().build()?;
        let mut stream = TcpStream::connect((self.host.clone(), self.port))?;
        let mut ssl_stream = connector.connect(&self.host, TcpStream::connect((self.host.clone(), self.port))?)?;

        stream.set_read_timeout(Some(time::Duration::from_secs(self.timeout)))?;
        stream.set_write_timeout(Some(time::Duration::from_secs(self.timeout)))?;

        if let Some(ref proxy) = self.proxy {
            if proxy.0.scheme == "http" {
                Self::write_all(&mut stream, self.body.as_ref(), header.as_bytes())
            } else {
                let connect_header = format!("CONNECT {host}:{port} HTTP/1.1\r\nHost: {host}:{port}\r\n\r\n",
                    host = self.host,
                    port = self.port);
                
                let res = Self::write_all(&mut stream, None, connect_header.as_bytes())?;

                if !res.text().contains("connection established") {
                    return Err(HttpError::Proxy("Proxy server response error."));
                }

                if self.scheme == "http" {
                    Self::write_all(&mut stream, self.body.as_ref(), header.as_bytes())
                } else {
                    Self::write_all(&mut ssl_stream, self.body.as_ref(), header.as_bytes())
                }
            }
        } else if self.scheme == "http" {
            Self::write_all(&mut stream, self.body.as_ref(), header.as_bytes())
        } else {
            Self::write_all(&mut ssl_stream, self.body.as_ref(), header.as_bytes())
        }
    }

    //build http request headers
    fn build_header(&self) -> String {
        let mut headers = format!("{method} {url} HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: Close\r\n",
            method = self.method,
            url = self.url.request_string(),
            host = self.host,
            port = self.port);

        if let Some(ref body) = self.body {
            headers.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }

        for (i, k) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", i, k));
        }

        headers.push_str("\r\n");
        headers
    }

    fn write_all<S>(stream: &mut S, body: Option<&Vec<u8>>, headers: &[u8]) -> Result<Response, HttpError>
    where 
        S: std::io::Read + std::io::Write,
    {
        stream.write_all(headers)?;
        if let Some(body) = body {
            stream.write_all(body)?;
        }
        stream.flush()?;

        let mut res: Vec<u8> = Vec::new();
        stream.read_to_end(&mut res)?;
        Ok(Response::new(res)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https_get() {
        let mut http = Client::new("https://docs.rs/").unwrap();
        println!(
            "{}",
            http.verify(false)
                .unwrap()
                .get()
                .send()
                .unwrap()
                .status_code()
        )
    }

    #[test]
    fn http_post() {
        let mut http = Client::new("https://docs.rs/").unwrap();
        println!(
            "{}",
            http
                .body("username=bob".as_bytes().to_vec())
                .post()
                .send()
                .unwrap()
                .status_code()
        )
    }

    #[test]
    fn http_get_set_header() {
        let mut http = Client::new("https://docs.rs/").unwrap();
        println!(
            "{}",
            http.headers(vec![("Content-Type".to_string(), "text/html; charset=utf-8".to_string())]).get().send().unwrap().status_code()
        )
    }

    #[test]
    fn http_get_back_header() {
        let mut http = Client::new("https://docs.rs/").unwrap();
        let headers = http.get().send().unwrap().headers();
        for (k, v) in headers {
            println!("{}:{}", k, v);
        }
    }

    #[test]
    fn http_proxy() {
        let mut http = Client::new("https://docs.rs/").unwrap();
        let res = http
            .proxy("https://127.0.0.1:1080")
            .unwrap()
            .get()
            .send()
            .unwrap();
        println!("{}", res.status_code());
    }

}
