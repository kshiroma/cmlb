use std::fmt::Pointer;
use std::fs::read;
use std::io::prelude::*;
use std::net::TcpStream;
use std::rc::Rc;

use crate::server::http_response::HttpResponseInfo;

pub struct Downstream {
    response: HttpResponseInfo,
    //writer: Rc<Write>,
}

impl Downstream {
    pub fn new(response: HttpResponseInfo) -> Self {
        let downstream = Downstream {
            response
        };
        return downstream;
    }

    pub fn sendFirstLine(&self, writer: &mut Write) {
        writer.write(self.response.http_first_line.protocol_version.as_bytes());
        writer.write(b" ");
        writer.write(self.response.http_first_line.http_status_code.to_string().as_bytes());
        writer.write(b" ");
        writer.write(self.response.http_first_line.http_status.as_bytes());
        writer.write(b"\r\n");
    }

    pub fn sendHeaders(&self, writer: &mut Write) {
        if self.response.http_response_header.keep_alive {}
        if self.response.http_response_header.content_length > 0 {
            writer.write(b"Content-Length: ");
            writer.write(self.response.http_response_header.content_length.to_string().as_bytes());
            writer.write(b"\r\n");
        }
        let response = &self.response;
        for header in &response.http_response_header.headers {
            let name = &header.name;
            let value = &header.value;
            writer.write(name.as_bytes());
            writer.write(b": ");
            writer.write(value.as_bytes());
            writer.write(b"\r\n");
        }
        writer.write(b"\r\n");
        log::trace!("end send response header.")
    }

    pub fn sendBody(&self, reader: &mut Read, writer: &mut Write) {
        log::trace!("start sendBody");
        let data_length = self.response.http_response_header.content_length;
        log::trace!("let data_length = self.response.http_response_header.content_length;");
        let mut buf = [0; 4096];
        log::trace!(stringify!(let mut buf = [0; 4096];));
        if data_length > 0 {
            log::trace!("enter data_length>0");
            let mut unsent_data_length = self.response.http_response_header.content_length;
            log::trace!("unsent_data_length is {}",unsent_data_length);
            while unsent_data_length > 0 {
                let size = reader.read(&mut buf).unwrap();
                let d = size.to_string();
                let read_length: i64 = d.parse().unwrap();
                writer.write(&buf[0..size]);
                log::trace!("response {} data",String::from_utf8_lossy(&buf[0..31]));
                unsent_data_length = unsent_data_length - read_length;
                log::trace!("unsent_data_length is {}",unsent_data_length);
            }
        } else if data_length == 0 {
            //何もしない
            log::trace!("response nothing");
        } else {
            let mut send_data_length = 0;
            log::trace!("enter data_length = 0");
            loop {
                log::trace!("reader.read(&mut buf).unwrap()");
                //let size = reader.read(&mut buf).unwrap();
                let size = reader.read(&mut buf).unwrap();
                let d = size.to_string();
                let data_length: i64 = d.parse().unwrap();
                writer.write(&buf[0..size]);
                log::trace!("response data_length = 0 :{} ",&buf[size-1]);
                send_data_length = send_data_length + data_length;
                writer.flush();
                send_data_length = 0;
                if size != buf.len() && buf[size - 1] == 10 {
                    break;
                }
            }
        }
    }
}

