extern crate reqwest;
extern crate clap;

use clap::{Arg, App};
use std::process::exit;

static APP_NAME: &'static str = "TinyUrl url shortener CLI";
static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "Valerio Casalino <casalinovalerio.cv@gmail.com>";
static DESCRIPTION: &'static str = "CLI Rust wrapper for tinyurl's API";

fn main() {
    // https://docs.rs/clap/2.33.0/clap/
    let matches = App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("input")
                .help("url 🔗 to shorten 😎")
                .required(true)
                .multiple(true))
        .arg(
            Arg::with_name("custom")
                .short("c")
                .long("custom")
                .multiple(false)
                .required(false)
                .takes_value(true)
                .help("For now it doesn't work 😪")
        )
        .get_matches();

    match matches.occurrences_of("input") {
        1 => println!("Shortening {}...", matches.value_of("input").unwrap()),
        _ => {eprintln!("You need just 1 link..."); exit(1); },
    }

    let to_shorten = matches.value_of("input").unwrap();
    let url = format!("https://tinyurl.com/api-create.php?url={}", to_shorten);

    match reqwest::get(url.as_str()) {
        Ok(mut response) => {
            // Check if 200OK
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("{}", text),
                    Err(_) => eprintln!("Cannot read response text"),
                }
            } else {
                eprintln!("Response wasn't 200OK");
            }
        },
        Err(_) => eprintln!("Could not make request"),
    }
}
