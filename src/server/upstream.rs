use std::borrow::Borrow;
use std::io::prelude::*;
use std::net::TcpStream;
use std::rc::Rc;



use crate::io::read_line;
use crate::server::config::RelayConnectionInfo;
use crate::server::http_request::{HttpRequestInfo, read_http_request};
use crate::server::http_response::{HttpResponseFirstLine, HttpResponseHeader, HttpResponseInfo, read_header};
use std::time::Duration;

pub struct Upstream {
    relay: Rc<RelayConnectionInfo>,
    request: Rc<HttpRequestInfo>,
    pub stream: TcpStream,
}

impl Upstream {
    pub fn new(relay: Rc<RelayConnectionInfo>, request: Rc<HttpRequestInfo>) -> Option<Upstream> {
        let result: std::io::Result<TcpStream> = relay.connect_relay();
        if result.is_err() {
            return None;
        }
        let stream = result.unwrap();
        let upstream = Upstream {
            relay,
            request,
            stream,
        };
        return Some(upstream);
    }

    pub fn sendFirstLine(&self) {
        let mut stream = &self.stream;
        stream.write(self.request.http_first_line.method.as_bytes());
        stream.write(b" ");
        stream.write(self.request.http_first_line.uri.as_bytes());
        stream.write(b" ");
        stream.write(self.request.http_first_line.protoolVersion.as_bytes());
        stream.write(b"\r\n");

        log::trace!("{}", self.request.http_first_line.method);
        log::trace!("{}", self.request.http_first_line.uri);
        log::trace!("{}", self.request.http_first_line.protoolVersion);
    }

    pub fn sendHeader(&self) {
        let mut stream = &self.stream;
        //Host
        println!("send host {}", self.relay.host.is_empty());
        if self.relay.host.is_empty() == false {
            stream.write(b"Host: ");
            stream.write(self.relay.host.as_bytes());
            stream.write(b"\r\n");
            println!("end send host.")
        }
        let a = self.request.clone();
        let request = &a;
        //Connection
        if request.http_request_header.keep_alive {}
        if request.http_request_header.content_length > 0 {
            stream.write(b"Content-Length: ");
            stream.write(request.http_request_header.content_length.to_string().as_bytes());
            stream.write(b"\r\n");
        }
        //ヘッダー
        let headers = &a.http_request_header.headers;
        for header in headers {
            let name = &header.name;
            let value = &header.value;
            stream.write(name.as_bytes());
            stream.write(b": ");
            stream.write(value.as_bytes());
            stream.write(b"\r\n");
        }
        stream.write(b"\r\n");
        log::trace!("end send header.")
    }

    pub fn sendBody(&self, reader: &mut Read) {
        let mut unsend_data_length = self.request.http_request_header.content_length;
        let mut buf = [0; 4096];
        while unsend_data_length > 0 {
            let size = reader.read(&mut buf).unwrap();
            let d = size.to_string();
            let data_length: i64 = d.parse().unwrap();
            self.send(&buf[0..size]);
            log::trace!("request {} bytes",d);
            unsend_data_length = unsend_data_length - data_length;
        }
    }


    pub fn send(&self, buf: &[u8]) {
        let mut stream = &self.stream;
        stream.write(buf);
    }
    pub fn flush(&self) {
        let mut stream = &self.stream;
        stream.flush();
    }


    pub fn read_http_response_info(&mut self) -> std::io::Result<HttpResponseInfo> {
        let mut read = &self.stream;
        return crate::server::http_response::read_http_response_info(&mut read);
    }
}