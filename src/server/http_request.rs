use std::io;
use std::io::prelude::*;

use crate::io::*;

use crate::http::http_header::{parse, http_header_entry};

pub struct http_request_info {
    pub http_first_line: http_request_first_line,
    pub http_request_header: http_request_header,
}

impl http_request_info {
    fn new(first_line_string: http_request_first_line, header_lines: http_request_header) -> Self {
        http_request_info {
            http_first_line: first_line_string,
            http_request_header: header_lines,
        }
    }
}

pub struct http_request_first_line {
    pub method: String,
    pub uri: String,
    pub protool_version: String,
    pub request: String,
}


impl http_request_first_line {
    pub fn new(first_line: String) -> Self {
        let mut array = first_line.split_whitespace();

        http_request_first_line {
            method: String::from(array.next().unwrap()),
            uri: String::from(array.next().unwrap()),
            protool_version: String::from(array.next().unwrap()),
            request: first_line,
        }
    }
}


pub struct http_request_header {
    pub host: String,
    pub content_length: i64,
    pub keep_alive: bool,
    pub headers: Vec<http_header_entry>,
}


impl http_request_header {
    pub fn empty() -> std::io::Result<Self> {
        let headers0: Vec<http_header_entry> = Vec::new();
        return Ok(http_request_header {
            host: "".to_string(),
            content_length: -1,
            keep_alive: false,
            headers: headers0,
        });
    }

    pub fn new(header_lines: Vec<String>) -> std::io::Result<Self> {
        let mut e = http_request_header::empty()?;
        for line in header_lines {
            e.add_string(line)?;
        }
        return Ok(e);
    }

    pub fn add_string(&mut self, header_line: String) -> std::io::Result<()> {
        if header_line.is_empty() {
            return Ok(());
        }
        let header = parse(header_line).expect("Bad_Request");
        if header.name.eq_ignore_ascii_case("Host") {
            if(self.host.is_empty()){
                self.host = header.value;
            }
        } else if header.name.eq_ignore_ascii_case("X-Forwarded-Host") {
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

pub fn read_http_request(reader: &mut Read) -> io::Result<http_request_info> {
    let first_line_string = read_line(reader);
    let first_line = http_request_first_line::new(first_line_string);
    log::trace!("{}", "begin read header");
    let headers = read_header(reader).unwrap();
    log::trace!("read {} headers", headers.headers.len());
    return Ok(http_request_info::new(first_line, headers));
}

pub fn read_header(reader: &mut Read) -> std::io::Result<http_request_header> {
    let mut headers: http_request_header = http_request_header::empty()?;
    loop {
        let line = read_line(reader);
        if line.is_empty() {
            break;
        }
        headers.add_string(line)?;
    }
    return Ok(headers);
}


#[test]
fn test_http_request_request_header() {
    let vec = vec![
        "Host: localhost".to_string(),
        "X-Forwarded-Host: locallocalh".to_string(),
        "User-Agent: curl/7.55.1".to_string(),
        "Accept: */*".to_string(),
        "Content-Length: 7".to_string(),
        "Connection: keep-alive".to_string(),
        "Content-Type: application/x-www-form-urlencoded".to_string(),
        "Content-Type: aaa:bbb".to_string(),
    ];
    let header = http_request_header::new(vec).unwrap();
    let mut a = "".to_string();
    a.push_str("aaa");

    println!("HOST : {} ", header.host);
    println!("KeepAlive: {} ", header.keep_alive);
    println!("content_length: {} ", header.content_length);
}

#[test]
fn test_read_first_line() -> std::io::Result<()> {
    use std::fs;
    use std::fs::File;
//use std::io::Read;
    let path = "test/httprequest/requets_post.txt";
    let _string = fs::read_to_string(path).unwrap();

    let mut reader = File::open(path).unwrap();
    let first_line = read_line(&mut reader);
    assert_eq!(first_line, "POST /bbb/ddd HTTP/1.1");

    //let headers = read_header(&mut reader);

    //println!("{}", body);

    return Ok(());
}