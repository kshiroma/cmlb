use std::net::TcpStream;

use crate::server::http_request::http_request_info;

pub struct routing_rule {
    name: String,
    routing_rule: fn(&http_request_info) -> Option<relay_connection_info>,
}

pub struct relay_connection_info {
    pub host: String,
    pub port: i32,
    pub path: String,
}

impl relay_connection_info {
    pub fn get_address(&self) -> String {
        let mut host = (&self.host).to_string();
        let port = &self.port;
        let port = *(port);
        if port > 1 && port != 80 {
            host.push(':');
            host = host + &port.to_string();
        }
        return host;
    }

    pub fn connect_relay(&self) -> std::io::Result<TcpStream> {
        let host = self.get_address();
        return std::net::TcpStream::connect(host);
    }
}

impl routing_rule {
    pub fn new(name: String, routing_rule: fn(&http_request_info) -> Option<relay_connection_info>) -> Self {
        routing_rule {
            name,
            routing_rule,
        }
    }

    pub fn route(&self, requet: &http_request_info) -> Option<relay_connection_info> {
        let func: fn(&http_request_info) -> Option<relay_connection_info> = self.routing_rule;
        return func(requet);
    }
}

pub struct server_config {
    routing_rules: Vec<routing_rule>,
}

impl server_config {
    pub fn new() -> Self {
        let vec: Vec<routing_rule> = Vec::new();
        server_config {
            routing_rules: vec
        }
    }

    pub fn add(&mut self, rule: routing_rule) {
        self.routing_rules.push(rule);
    }

    pub fn find_routing_rule(&self, request: &http_request_info) -> Option<&routing_rule> {
        for rule in self.routing_rules.iter() {
            if let Some(_) = (rule.routing_rule)(request) {
                return Some(rule);
            }
        }
        return None;
    }

    pub fn route(&self, request: &http_request_info) -> Option<relay_connection_info> {
        for rule in self.routing_rules.iter() {
            log::trace!("checking {}", rule.name);
            if let Some(r) = (rule.routing_rule)(request) {
                return Some(r);
            }
        }
        return None;
    }
}
