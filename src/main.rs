extern crate rand;
extern crate regex;

use std::env;

use env_logger;

use crate::routing_sample::create_sample_config;

use log::{debug, error, info, warn};

use crate::routing_sample::createSampleConfig;

//pub mod study;
pub mod http;
pub mod server;
//pub mod study;
pub mod io;
pub mod routing_sample;


fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
//    log::info!("info");
//    log::warn!("warn");
//    log::debug!("debug");
//    log::error!("error");
//    log::trace!("trace");

    let config = createSampleConfig();
    server::listen(config, 80).unwrap();
}
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::ops::Deref;

static NTHREADS: i32 = 30;

#[test]
fn test(){
    let (tx, tr): (Sender<i32>, Receiver<i32>) = mpsc::channel();


    let mut children = Vec::new();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        let child = thread::spawn(move || {
            thread_tx.send(id);
            log::info!("thread {} finished", id);
        });
        children.push(child);
    }
    for child in children {
        child.join().unwrap();
    }
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for i in 0..NTHREADS {
        log::info!("push {}", i);
        ids.push(tr.recv());
    }

    println!("{:?}", ids);
}

#[test]
fn aho(){
    let mut guess = "aaabbb".to_string();
    let a = &guess;
    //guess.remove(1);//moveして
    //a.remove(1);
    //guess = a ;
    // この時点で返却している
    guess.remove(1);
    //guess.remove(1);
    //a.remove(1);
    println!("{}",guess);//ここではエラー
    //println!("{}",a);

    //println!("{}",a);
}