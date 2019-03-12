use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};

use regex::Regex;

use crate::server::config::{RelayConnectionInfo, RoutingRule, ServerConfig};
use crate::server::http_request::HttpRequestInfo;

pub fn createSampleConfig() -> ServerConfig {
    let routeingRule1 = RoutingRule::new("routing1".to_string(), routing1);
    let routeingRule2 = RoutingRule::new("routing2".to_string(), routing2);
    let mut config = ServerConfig::new();
    config.add(routeingRule1);
    config.add(routeingRule2);

    return config;
}


pub fn routing1(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let re = Regex::new(r"^/cattleya/.*").unwrap();
    let mut path = "none";
    let relay = if re.is_match(&request.http_first_line.uri) {
        path = "/cattleya";
        Some(RelayConnectionInfo {
            host: "localhost".to_string(),
            port: 8000,
            path: path.to_string(),
        })
    } else {
        None
    };
    print("routing1", request, &path);
    return relay;
}

pub fn routing2(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let re = Regex::new(r"^/bbb/.*").unwrap();
    let mut path = "none";
    let relay = if re.is_match(&request.http_first_line.uri) {
        path = "/ccc/ddd";
        Some(RelayConnectionInfo {
            host: "localhost".to_string(),
            port: 8001,
            path: path.to_string(),
        })
    } else {
        None
    };
    print("routing2", request, path);

    return relay;
}

fn print(name: &str, request: &HttpRequestInfo, path: &str) {
    println!(" checking {} ... {} ,{}", name, request.http_first_line.request, path);
}

pub fn routing3(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let re = Regex::new(r"^/s2cattleya/.*").unwrap();
    let mut path = "none";
    let username = std::env::var_os("USERNAME").unwrap().to_str().unwrap();
    let relay = if re.is_match(&request.http_first_line.uri) {
        path = "/cattleya";
        Some(RelayConnectionInfo {
            host: "localhost".to_string(),
            port: 8000,
            path: path.to_string(),
        })
    } else {
        None
    };
    print("routing1", request, &path);
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
fn test_get_user_name() {
}