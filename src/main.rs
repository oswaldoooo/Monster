use once_cell::sync::Lazy;

use crate::api::app_run;
use std::io::Read;

mod api;
mod config;
#[path = "core/mod.rs"]
mod core;
#[path = "tests/node_test.rs"]
mod tests;
use lazy_static::lazy_static;
lazy_static! {
    static ref global_core: crate::core::Core = crate::core::Core::new();
}
fn main() {
    let mut cnf = load_config("./config.json");
    println!("listen at {}", cnf.port);
    app_run(cnf.port);
    // let svc=warp::path("hello").
    //     map(||"hello world");
    // warp::serve(svc).run(([0,0,0,0],1999)).await;
    println!("programmer exit!");
}
fn load_config(rpath: &'static str) -> config::Config {
    let mut fio = std::fs::File::open(rpath).unwrap();
    return serde_json::from_reader(fio).unwrap();
    // return config::Config::default();
}
