use std::borrow::Borrow;
use std::io::Read;
use std::net::TcpStream;

use regex::Regex;

use crate::http::http_header::http_header_entry;
use crate::server::config::{relay_connection_info, routing_rule, server_config};
use crate::server::http_request::http_request_info;

pub fn create_sample_config() -> server_config {
    let mut config = server_config::new();
    //config.add(RoutingRule::new("odj".to_string(), routing_odj));
    //config.add(RoutingRule::new("wakuden".to_string(), routing_wakuden));
    //config.add(RoutingRule::new("timer".to_string(), routing_timer));
    config.add(routing_rule::new("md".to_string(), routing_milliondollar));
    return config;
}

fn routing_odj(request: &http_request_info) -> Option<relay_connection_info> {
    routing("odj", request)
}

fn routing_wakuden(request: &http_request_info) -> Option<relay_connection_info> {
    routing("wakuden", request)
}

fn routing(prefix: &str, request: &http_request_info) -> Option<relay_connection_info> {
    let path = "/cattleya";
    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = if request.http_first_line.uri.contains('?') { "&" } else { "?" };
    let relay = if host.starts_with(pattern) {
        Some(relay_connection_info {
            host: "dev-jt0001".to_string(),
            port: 8000,
            path: path.to_string() + conjunction + "targetUser=" + prefix,
        })
    } else {
        None
    };
    return relay;
}

//pub fn routing2(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
//    let username = std::env::var_os("USERNAME").map(|s| s.into_string()).unwrap().unwrap();
//    let path = "/".to_string() + username.as_str() + "_zenrou-s2";
//    log::trace!("{} {}",request.http_first_line.uri,path);
//    let relay = if request.http_first_line.uri.starts_with(path.as_str()) {

fn routing_milliondollar(request: &http_request_info) -> Option<relay_connection_info> {
    let prefix: &str = "million-dollar";

    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = &request.http_first_line.uri;
    let relay = if host.starts_with(pattern) {
        Some(relay_connection_info {
            host: "localhost".to_string(),
            port: 1234,
            path: "".to_string() + conjunction,
        })
    } else {
        None
    };
    return relay;
}


fn routing_timer(request: &http_request_info) -> Option<relay_connection_info> {
    let prefix: &str = "timer";
    let path = "/cattleya";
    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = if request.http_first_line.uri.contains('?') { "&" } else { "?" };
    let relay = if host.starts_with(pattern) {
        Some(relay_connection_info {
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
    use std::io::Read;
    use std::io::Write;
    let relay = relay_connection_info {
        host: "localhost".to_string(),
        port: 8080,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    println!("relay host is {}", relay.get_address());

    let mut stream = &relay.connect_relay().unwrap();
    stream.write(b"GET ").unwrap();
    stream.write(&relay.path.into_bytes()).unwrap();
    stream.write(b" HTTP/1.1\r\n").unwrap();

    stream.write(b"Host: localhost:8000\r\n").unwrap();
    stream.write(b"User-Agent: curl/7.55.1\r\n").unwrap();
    stream.write(b"Accept: */*\r\n\r\n").unwrap();
    stream.flush().unwrap();
    let mut data = [0; 4096];
    stream.read(&mut data).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}


#[test]
fn test_get_address() {
    let relay = relay_connection_info {
        host: "localhost".to_string(),
        port: 8080,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    assert_eq!("localhost:8080", relay.get_address());
    let relay = relay_connection_info {
        host: "localhost".to_string(),
        port: 80,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    assert_eq!("localhost", relay.get_address());
    let relay = relay_connection_info {
        host: "localhost".to_string(),
        port: 0,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
    };
    assert_eq!("localhost", relay.get_address());
}


#[test]
fn test_get_user_name() {}