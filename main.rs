extern crate serde_json;
extern crate chrono;
extern crate env_logger;
extern crate openssl;
extern crate regex;
extern crate rusqlite;
extern crate select;
extern crate bytes;

mod crawler;
mod database;
mod fuzzer;
mod http;

use std::io;

fn main() {

    println!("\n\n################################");
    println!("WELCOME TO ATOSSA DEMONSTRATION!");
    println!("################################\n");

    // println!("\n==> Type an address for crawling (www.website.domain):");
    //
    // let mut url = String::new();
    //
    // io::stdin().read_line(&mut url).expect("Failed to read line!");
    // let url = url.trim();
    //
    // println!("\n==> Starting crawler process for {}\n", url);
    //
    // use crawler::Agent;
    // let mut _agent = Agent::new();
    // _agent.crawl(url.to_string(), 1);
    //
    // println!("\n==> Crawler process finished successfuly.");

    // println!("\n==> Type an address for fuzzing (www.website.domain):");
    //
    // let mut url = String::new();
    //
    // io::stdin().read_line(&mut url).expect("Failed to read line!");
    // let url = url.trim();
    //
    // println!("\n==> Starting fuzzer process for {}\n", url);
    //
    // use fuzzer::Fuzzer;
    // use fuzzer::Option;
    // let mut _fuzzer = Fuzzer::new(Option::DIRECTORY_SEARCH);
    // _fuzzer.execute(url.to_string());
    //
    // println!("\n==> Fuzzer process finished successfuly.");

    // iw-microwave.com/.git/
}
