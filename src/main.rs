extern crate colored;
extern crate reqwest;

use colored::*;

mod http_client;
mod decoder;
mod repl;
mod interpreter;
mod command;

fn main() {
    println!("{}", "\nInteractive influxdb tool \n".yellow());

    repl::start();

    // let url = String::from("http://localhost:8086");
    // let db = String::from("prices");

    // // http_client::post(String::from("https://jsonplaceholder.typicode.com/users"));
    // match http_client::get(String::from("http://localhost:8086/query?db=prices&q=show%20tag%20keys")) {
    //   Ok(res) => {
    //     match decoder::string_to_json(res) {
    //       Ok(json_decoded) =>
    //         println!("{:?}", json_decoded),
    //       Err(err) =>
    //         println!("Error decoding json"),
    //     }
    //   },

    //   Err(error) =>
    //     println!("Error fetching! {:?}", error)
    // }
}
