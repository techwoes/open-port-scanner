#[macro_use]
extern crate clap;
extern crate opslib;
extern crate log;
extern crate ipnetwork;
extern crate iprange;
extern crate ipnet;
extern crate range_map;
mod cfg;

use cfg::parse_args;


fn main() {
    let cfg = parse_args();
    println!("Configuration: {:?}", cfg);
}
