extern crate rand;
extern crate regex;

use std::env;

use env_logger;
//use log::{debug, error, info, warn};

use crate::routing_sample::create_sample_config;

//pub mod study;
pub mod http;
pub mod server;
//pub mod study;
pub mod io;
pub mod routing_sample;


fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    log::info!("info");
    log::warn!("warn");
    log::debug!("debug");
    log::error!("error");
    log::trace!("trace");

    let config = create_sample_config();
    server::listen(config, 80).unwrap();
}
