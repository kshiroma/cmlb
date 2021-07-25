//use std::borrow::Borrow;
//use std::io::Read;
//use std::net::TcpStream;

//use regex::Regex;

//use crate::http::http_header::HttpHeaderEntry;
use crate::server::config::{RelayConnectionInfo, RoutingRule, ServerConfig};
use crate::server::http_request::HttpRequestInfo;

pub fn create_sample_config() -> ServerConfig {
    let mut config = ServerConfig::new();
    //config.add(RoutingRule::new("ok".to_string(), ok));
    config.add(RoutingRule::new("set_routing_number".to_string(), set_routing_number));
    config.add(RoutingRule::new("routing".to_string(), routing));
    //config.add(RoutingRule::new("timer".to_string(), routing_timer));
    //config.add(RoutingRule::new("md".to_string(), routing_milliondollar));
    return config;
}

fn set_routing_number(config: &ServerConfig, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let path = "/set_routing_number";
    if (request.http_first_line.uri.starts_with(path)) {
        let number = request.http_first_line.uri.replace(path,"");
        let number = from_str(number);
        config.set_routing_number(number);
    }
    return None;
}

fn routing(config: &ServerConfig, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let path = "/cattleya";

    let relay = if true {
        let i = config.get_count();
        let n = config.get_routing_number();
        println!("connt {}", i);
        match n {
            0 =>
                if i % 2 == 0 {
                    Some(RelayConnectionInfo {
                        host: "localhost".to_string(),
                        port: 8000,
                        path: path.to_string(),
                        relayInfo: "0_0".to_string(),
                    })
                } else {
                    Some(RelayConnectionInfo {
                        host: "localhost".to_string(),
                        port: 8000,
                        path: path.to_string(),
                        relayInfo: "0_1".to_string(),
                    })
                },
            1 =>
                Some(RelayConnectionInfo {
                    host: "localhost".to_string(),
                    port: 8000,
                    path: path.to_string(),
                    relayInfo: "1__".to_string(),
                }),
            _ =>
                Some(RelayConnectionInfo {
                    host: "localhost".to_string(),
                    port: 8000,
                    path: path.to_string(),
                    relayInfo: "___".to_string(),
                }),
        }
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
            relayInfo: "million_dollar".to_string(),
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
            relayInfo: "chronotrigger".to_string(),
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
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 8080,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
        relayInfo: "test".to_string(),
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
    let mut data = [0; 4096 * 4];
    stream.read(&mut data).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}


#[test]
fn test_get_address() {
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 8080,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
        relayInfo: "test".to_string(),
    };
    assert_eq!("localhost:8080", relay.get_address());
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 80,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
        relayInfo: "test".to_string(),
    };
    assert_eq!("localhost", relay.get_address());
    let relay = RelayConnectionInfo {
        host: "localhost".to_string(),
        port: 0,
        path: "/cattleya/view/login?targetUser=wakuden".to_string(),
        relayInfo: "test".to_string(),
    };
    assert_eq!("localhost", relay.get_address());
}


#[test]
fn test_get_user_name() {}