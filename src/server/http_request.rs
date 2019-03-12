use std::io;
use std::io::prelude::*;
use std::ops::Deref;

use crate::io::*;

use crate::http::http_header::{parse, HttpHeaderEntry};

pub struct HttpRequestInfo {
    pub http_first_line: HttpRequestFirstLine,
    pub http_request_header: HttpRequestHeader,
}

impl HttpRequestInfo {
    fn new(firstLineString: HttpRequestFirstLine, headerLines: HttpRequestHeader) -> Self {
        HttpRequestInfo {
            http_first_line: firstLineString,
            http_request_header: headerLines,
        }
    }
}

pub struct HttpRequestFirstLine {
    pub method: String,
    pub uri: String,
    pub protoolVersion: String,
    pub request: String,
}


impl HttpRequestFirstLine {
    pub fn new(firstLine: String) -> Self {
        let mut array = firstLine.split_whitespace();

        HttpRequestFirstLine {
            method: String::from(array.next().unwrap()),
            uri: String::from(array.next().unwrap()),
            protoolVersion: String::from(array.next().unwrap()),
            request: firstLine,
        }
    }
}


pub struct HttpRequestHeader {
    pub host: String,
    pub content_length: i64,
    pub keep_alive: bool,
    pub headers: Vec<HttpHeaderEntry>,
}


impl HttpRequestHeader {
    pub fn empty() -> std::io::Result<Self> {
        let headers0: Vec<HttpHeaderEntry> = Vec::new();
        return Ok(HttpRequestHeader {
            host: "".to_string(),
            content_length: -1,
            keep_alive: false,
            headers: headers0,
        });
    }

    pub fn new(headerLines: Vec<String>) -> std::io::Result<Self> {
        let mut e = HttpRequestHeader::empty()?;
        for line in headerLines {
            e.add_string(line)?;
        }
        return Ok(e);
    }

    pub fn add_string(&mut self, headerLine: String) -> std::io::Result<()> {
        if headerLine.is_empty() {
            return Ok(());
        }
        let header = parse(headerLine).expect("Bad_Request");
        if header.name.eq_ignore_ascii_case("Host") {
            self.host = header.value;
        } else if header.name.eq_ignore_ascii_case("Content-Length") {
            self.content_length = header.value.parse().unwrap_or(-1);
        } else if header.name.eq_ignore_ascii_case("Connection") {
            if header.value.eq_ignore_ascii_case("keep-alive") {
                self.keep_alive = true;
            }
        } else {
            self.headers.push(header);
        }
        return Ok(());
    }
}

pub fn read_http_request(reader: &mut Read) -> io::Result<HttpRequestInfo> {
    let firstLineString = read_line(reader);
    let firstLine = HttpRequestFirstLine::new(firstLineString);
    println!("{}", "begin read header");
    let headers = read_header(reader).unwrap();
    println!("read {} headers", headers.headers.len());
    return Ok((HttpRequestInfo::new(firstLine, headers)));
}

pub fn read_header(reader: &mut Read) -> std::io::Result<HttpRequestHeader> {
    let mut headers: HttpRequestHeader = HttpRequestHeader::empty()?;
    loop {
        let line = read_line(reader);
        if line.is_empty() {
            break;
        }
        headers.add_string(line);
    }
    return Ok(headers);
}


#[test]
fn test_HttpRequestRequestHeadr() {
    let vec = vec![
        "Host: localhost".to_string(),
        "User-Agent: curl/7.55.1".to_string(),
        "Accept: */*".to_string(),
        "Content-Length: 7".to_string(),
        "Connection: keep-alive".to_string(),
        "Content-Type: application/x-www-form-urlencoded".to_string(),
        "Content-Type: aaa:bbb".to_string(),
    ];
    let header = HttpRequestHeader::new(vec).unwrap();
    println!("HOST : {} ", header.host);
    println!("KeepAlive: {} ", header.keep_alive);
    println!("content_length: {} ", header.content_length);
}

#[test]
fn test_readFirstLine() -> std::io::Result<()> {
    use std::fs;
    use std::fs::File;
//use std::io::Read;
    let path = "test/httprequest/requets_post.txt";
    let _string = fs::read_to_string(path).unwrap();

    let mut reader = File::open(path).unwrap();
    let firstLine = read_line(&mut reader);
    assert_eq!(firstLine, "POST /bbb/ddd HTTP/1.1");

    //let headers = read_header(&mut reader);

    //println!("{}", body);

    return Ok(());
}