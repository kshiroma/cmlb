use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::sync::Arc;

use crate::http::http_status::not_found;
use crate::server::config::{relay_connection_info, server_config};
use crate::server::downstream::downstream;
use crate::server::http_request::read_http_request;
use crate::server::upstream::Upstream;

pub struct Worker {
    config: Arc<server_config>,
}

impl Worker {
    pub fn new(config: Arc<server_config>) -> Self {
        Worker {
            config,
        }
    }

    pub fn handle(&self, mut _stream: TcpStream) -> std::io::Result<()> {
        let b = Box::new(_stream);
        let mut reader = b.try_clone()?;
        let mut writer = b.try_clone().unwrap();
        let stream = b.try_clone().unwrap();
        //
        self.handle_read_writer(&mut reader, &mut writer)?;
        //終わり
        writer.flush().unwrap();
        stream.shutdown(Shutdown::Both).unwrap();
        //reader.shutdown(Shutdown::Both);
        log::trace!("shutdown stream");
        return Ok(());
    }

    fn handle_read_writer(&self, reader: &mut Read, writer: &mut Write) -> std::io::Result<()> {
        let request = read_http_request(reader)?;
        let relay: Option<relay_connection_info> = self.config.route(&request);
        if relay.is_none() {
            log::info!("not found relay connection {}", request.http_first_line.uri);
            not_found(writer).unwrap();
            return Ok(());
        }
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
        let downstream = downstream::new(response_info);
        log::trace!("let downstream = Downstream::new(response_info);");
        downstream.send_first_line(writer);
        log::trace!("downstream.sendFirstLine(writer);");
        downstream.send_headers(writer);
        log::trace!("downstream.sendHeaders(writer);");
        downstream.send_body(&mut upstream.stream, writer);
        log::trace!("downstream.sendBody(&mut upstream.stream, writer);");
        writer.flush().unwrap();
        log::trace!("writer.flush();");
        return Ok(());
    }
}


