extern crate rand;
extern crate regex;

use std::env;

use env_logger;
use log::{debug, error, info, warn};

use crate::routing_sample::createSampleConfig;

pub mod study;
pub mod http;
pub mod server;
//pub mod study;
pub mod io;
pub mod routing_sample;


fn main() {
    let config = createSampleConfig();
    server::listen(config, 80).unwrap();
}
