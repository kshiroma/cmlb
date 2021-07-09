use std::io::BufReader;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::sync::Arc;

use crate::http::http_status::not_found;
use crate::server::config::{RelayConnectionInfo, ServerConfig};
use crate::server::downstream::Downstream;
use crate::server::http_request::read_http_request;
use crate::server::upstream::Upstream;
use std::ops::Deref;

pub struct Worker {
    config: Arc<ServerConfig>,
}

impl Worker {
    pub fn new(config: Arc<ServerConfig>) -> Self {
        Worker {
            config,
        }
    }

    pub fn handle(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut stream_box = Box::new(stream);
        let mut read = stream_box.try_clone().unwrap();
        let mut write = stream_box.try_clone().unwrap();
        let mut bufReader = BufReader::new(read);
        self.handle_read_writer(&mut bufReader, &mut write)?;
        //終わり
        stream_box.flush().unwrap();
        stream_box.shutdown(Shutdown::Both).unwrap();
        //reader.shutdown(Shutdown::Both);
        log::trace!("shutdown stream");
        return Ok(());
    }

    fn handle_read_writer(&self, reader: &mut dyn BufRead, writer: &mut dyn Write) -> std::io::Result<()> {
        let request = read_http_request(reader)?;
        let relay: Option<RelayConnectionInfo> = self.config.route(&request);
        if relay.is_none() {
            log::info!("not found relay connection {}", request.http_first_line.uri);
            not_found(writer).unwrap();
            return Ok(());
        }
        let count = self.config.count.deref() + 1;
        self.config.count.  count;

        let relay = relay.unwrap();
        log::info!("relay connection host is {}:{}", relay.host, relay.port);
        //
        let b_relay = std::rc::Rc::new(relay).clone();
        let b_request = std::rc::Rc::new(request).clone();
        let mut upstream = Upstream::new(b_relay, b_request).unwrap();

        upstream.send_first_line();
        log::trace!("upstream.sendFirstLine()");
        upstream.send_headers();
        log::trace!("upstream.sendHeader()");
        upstream.send_body(reader);
        log::trace!("upstream.sendBody(reader);");
        upstream.flush();
        log::trace!("upstream.flush();");
        let response_info = upstream.read_http_response_info().unwrap();
        log::trace!("let response_info = upstream.read_http_response_info().unwrap();");

        let downstream = Downstream::new(response_info);
        log::trace!("let downstream = Downstream::new(response_info);");
        downstream.send_first_line(writer);
        log::trace!("downstream.sendFirstLine(writer);");
        downstream.send_headers(writer);
        log::trace!("downstream.sendHeaders(writer);");
        downstream.send_body(&mut upstream.bufReader, writer);
        log::trace!("downstream.sendBody(&mut upstream.stream, writer);");
        writer.flush().unwrap();
        log::trace!("writer.flush();");
        return Ok(());
    }
}


