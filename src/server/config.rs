use std::net::TcpStream;

use crate::server::http_request::HttpRequestInfo;

pub struct RoutingRule {
    name: String,
    routing_rule: fn(&HttpRequestInfo) -> Option<RelayConnectionInfo>,
}

pub struct RelayConnectionInfo {
    pub host: String,
    pub port: i32,
    pub path: String,
}

impl RelayConnectionInfo {
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

impl RoutingRule {
    pub fn new(name: String, routing_rule: fn(&HttpRequestInfo) -> Option<RelayConnectionInfo>) -> Self {
        RoutingRule {
            name,
            routing_rule,
        }
    }

    pub fn route(&self, requet: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
        let func: fn(&HttpRequestInfo) -> Option<RelayConnectionInfo> = self.routing_rule;
        return func(requet);
    }
}

pub struct ServerConfig {
    routing_rules: Vec<RoutingRule>,
}

impl ServerConfig {
    pub fn new() -> Self {
        let vec: Vec<RoutingRule> = Vec::new();
        ServerConfig {
            routing_rules: vec
        }
    }

    pub fn add(&mut self, rule: RoutingRule) {
        self.routing_rules.push(rule);
    }

    pub fn find_routing_rule(&self, request: &HttpRequestInfo) -> Option<&RoutingRule> {
        for rule in self.routing_rules.iter() {
            if let Some(_) = (rule.routing_rule)(request) {
                return Some(rule);
            }
        }
        return None;
    }

    pub fn route(&self, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
        for rule in self.routing_rules.iter() {
            println!("checking {}", rule.name);
            if let Some(r) = (rule.routing_rule)(request) {
                return Some(r);
            }
        }
        return None;
    }
}
