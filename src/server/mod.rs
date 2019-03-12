use std::env;
use std::net::TcpListener;

use chrono::Duration;
use log::{debug, error, info, LevelFilter, warn};

use crate::routing_sample::createSampleConfig;
use crate::server::config::ServerConfig;

pub mod config;
pub mod http_request;
pub mod http_response;
mod worker;
mod upstream;
mod downstream;

pub fn listen(config: ServerConfig, port: i32) -> std::io::Result<()> {
    let rc = std::sync::Arc::new(config);
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))?;
    //listener.set_nonblocking(true);
    for stream in listener.incoming() {
        let rc0 = rc.clone();
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("An error occurred while accepting a connection:{}", e);
                continue;
            }
        };
        std::thread::spawn(move || -> std::io::Result<()> {
            debug!("worker start");
            let worker = worker::Worker::new(rc0);
            let result = worker.handle(stream);
            debug!("worker end.");
            return result;
        });
    }
    Ok(())
}

#[test]
fn test() {
    log::set_max_level(LevelFilter::Trace);
    let config = createSampleConfig();
    listen(config, 80);
}

