extern crate rand;
extern crate regex;

use crate::routing_sample::createSampleConfig;

pub mod study;
pub mod http;
pub mod server;
//pub mod study;
pub mod io;
pub mod routing_sample;


use log::{error, warn, info, debug};
use std::env;
use env_logger;
fn main() {
    //log::set_max_level(LevelFilter::Trace);
    //log::log_enabled!(true);
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    log::info!("info");
    log::warn!("warn");
    log::debug!("debug");
    log::error!("error");
    log::trace!("trace");


    let config = createSampleConfig();
    server::listen(config, 80);
}
