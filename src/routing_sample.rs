use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};

use regex::Regex;

use crate::http::http_header::HttpHeaderEntry;
use crate::server::config::{RelayConnectionInfo, RoutingRule, ServerConfig};
use crate::server::http_request::HttpRequestInfo;

pub fn createSampleConfig() -> ServerConfig {
    let mut config = ServerConfig::new();
    //config.add(RoutingRule::new("odj".to_string(), routing_odj));
    //config.add(RoutingRule::new("wakuden".to_string(), routing_wakuden));
    //config.add(RoutingRule::new("timer".to_string(), routing_timer));
    config.add(RoutingRule::new("md".to_string(), routing_milliondollar));
    return config;
}

fn routing_odj(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    routing("odj", request)
}

fn routing_wakuden(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    routing("wakuden", request)
}

fn routing(prefix: &str, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let path = "/cattleya";
    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = if request.http_first_line.uri.contains('?') { "&" } else { "?" };
    let relay = if host.starts_with(pattern) {
        Some(RelayConnectionInfo {
            host: "dev-jt0001".to_string(),
            port: 8000,
            path: path.to_string() + conjunction + "targetUser=" + prefix,
        })
    } else {
        None
    };
    return relay;
}

fn routing_milliondollar(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let prefix: &str = "million-dollar";

    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = &request.http_first_line.uri;
    let relay = if host.starts_with(pattern) {
        Some(RelayConnectionInfo {
            host: "localhost".to_string(),
            port: 1234,
            path: "".to_string() + conjunction,
        })
    } else {
        None
    };
    return relay;
}


fn routing_timer(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let prefix: &str = "timer";
    let path = "/cattleya";
    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = if request.http_first_line.uri.contains('?') { "&" } else { "?" };
    let relay = if host.starts_with(pattern) {
        Some(RelayConnectionInfo {
            host: "dev-timer".to_string(),
            port: 8000,
            path: path.to_string() + conjunction + "targetUser=" + prefix,
        })
    } else {
        None
    };
    return relay;
}


#[test]
fn test() {
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 8080,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    println!("relay host is {}", relay.get_address());

    let mut stream = &relay.connect_relay().unwrap();
    stream.write(b"GET ");
    stream.write(&relay.path.into_bytes());
    stream.write(b" HTTP/1.1\r\n");

    stream.write(b"Host: localhost:8000\r\n");
    stream.write(b"User-Agent: curl/7.55.1\r\n");
    stream.write(b"Accept: */*\r\n\r\n");
    stream.flush();
    println!("flush");
    let mut data = [0; 4096];
    stream.read(&mut data);
    println!("{}", String::from_utf8_lossy(&data));
}


#[test]
fn test_get_address() {
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 8080,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    assert_eq!("localhost:8080", relay.get_address());
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 80,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    assert_eq!("localhost", relay.get_address());
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 0,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    assert_eq!("localhost", relay.get_address());
}


#[test]
fn test_get_user_name() {}