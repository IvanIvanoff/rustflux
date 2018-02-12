extern crate chrono;
extern crate colored;
extern crate reqwest;

use colored::*;

mod errors;
mod http_client;
mod decoder;
mod repl;
mod interpreter;
mod command;
mod context;
mod queries;

fn main() {
    println!("{}", "\nInteractive influxdb tool \n".yellow());

    let _ = repl::start();
}
