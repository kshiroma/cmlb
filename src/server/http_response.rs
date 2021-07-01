use std::io::Read;

use crate::http::http_header::{http_header_entry, parse};
use crate::io::read_line;

pub struct http_response_info {
    pub http_first_line: http_response_first_line,
    pub http_response_header: http_response_header,
}

impl http_response_info {
    pub fn new(http_first_line: http_response_first_line, http_response_header: http_response_header) -> Self {
        http_response_info {
            http_first_line,
            http_response_header,
        }
    }
}


pub struct http_response_first_line {
    pub protocol_version: String,
    pub http_status_code: i32,
    pub http_status: String,
    pub resonse: String,
}

impl http_response_first_line {
    pub fn new(first_line: String) -> Self {
        let mut array = first_line.split_whitespace();

        http_response_first_line {
            protocol_version: String::from(array.next().unwrap_or_default()),
            http_status_code: String::from(array.next().unwrap()).parse().unwrap(),
            http_status: String::from(array.next().unwrap_or_default()),
            resonse: first_line,
        }
    }
}

pub struct http_response_header {
    pub content_length: i64,
    pub headers: Vec<http_header_entry>,
    pub keep_alive: bool,
}

impl http_response_header {
    pub fn empty() -> std::io::Result<Self> {
        let headers0: Vec<http_header_entry> = Vec::new();
        return Ok(http_response_header {
            content_length: -1,
            headers: headers0,
            keep_alive: false,
        });
    }

    pub fn new(header_lines: Vec<String>) -> std::io::Result<Self> {
        let mut e = http_response_header::empty()?;
        for line in header_lines {
            e.add_string(line)?;
        }
        return Ok(e);
    }

    pub fn add_string(&mut self, header_line: String) -> std::io::Result<()> {
        if header_line.is_empty() {
            return Ok(());
        }
        let header = parse(header_line).expect("Bad Request");
        if header.name.eq_ignore_ascii_case("Content-Length") {
            self.content_length = header.value.parse().unwrap_or(-1);
        } else if header.name.eq_ignore_ascii_case("Connection") {
            if header.value.eq_ignore_ascii_case("Content-Length") {}
        } else {
            self.headers.push(header);
        }
        return Ok(());
    }
}

pub fn read_header(reader: &mut Read) -> std::io::Result<http_response_header> {
    let mut headers: http_response_header = http_response_header::empty()?;
    loop {
        let line = read_line(reader);
        if line.is_empty() {
            break;
        }
        headers.add_string(line)?;
    }
    return Ok(headers);
}

pub fn read_http_response_info(read: &mut Read) -> std::io::Result<http_response_info> {
    let first_string = read_line(read);
    let str = first_string.clone();
    let first_line = http_response_first_line::new(first_string);
    println!("begin read response header of {}", str);
    let headers = read_header(read).unwrap();

    return Ok(http_response_info::new(first_line, headers));
}
//#[test]
//pub fn test_read_http_reponse() {
//    let path = "test/httpresponse/response_a.txt";
//    //let _string = std::fs::read_to_string(path).unwrap();
//    let mut reader = std::fs::File::open(path).unwrap();
//    let response = read_http_response(&mut reader).unwrap();
//    assert_eq!("OK", response.http_first_line.http_status);
//    assert_eq!(200, response.http_first_line.http_status_code);
//    assert_eq!("HTTP/1.1", response.http_first_line.protocol_version);
//
//    assert_eq!(5055, response.http_response_header.content_length)
//}