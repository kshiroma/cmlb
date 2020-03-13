use crate::server::config::{RelayConnectionInfo, RoutingRule, ServerConfig};
use crate::server::http_request::HttpRequestInfo;

pub fn create_sample_config() -> ServerConfig {
    let mut config = ServerConfig::new();
    config.add(RoutingRule::new("routing1".to_string(), routing1));
    config.add(RoutingRule::new("routing2".to_string(), routing2));
    return config;
}

pub fn routing1(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let path = "/cattleya";
    let relay = if request.http_first_line.uri.starts_with(path) {
        Some(RelayConnectionInfo {
            host: "localhost".to_string(),
            port: 8000,
            path: path.to_string(),
        })
    } else {
        None
    };
    return relay;
}

pub fn routing2(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let username = std::env::var_os("USERNAME").map(|s| s.into_string()).unwrap().unwrap();
    let path = "/".to_string() + username.as_str() + "_zenrou-s2";
    log::trace!("{} {}",request.http_first_line.uri,path);
    let relay = if request.http_first_line.uri.starts_with(path.as_str()) {
        Some(RelayConnectionInfo {
            host: "localhost".to_string(),
            port: 8083,
            path: path.to_string(),
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