extern crate chrono;
extern crate colored;
extern crate reqwest;

use colored::*;

mod errors;
mod http_client;
mod converter;
mod repl;
mod interpreter;
mod command;
mod context;
mod queries;
mod filesystem;

fn main() {
    println!("{}", "\nInteractive influxdb tool \n".yellow());

    match repl::start() {
        Err(err) => println!("Rustflux exited with error: {}", err),
        Ok(_) => println!("Exiting Rustflux"),
    }
}
