extern crate reqwest;
extern crate clap;

use clap::{Arg, App};
use std::process::exit;

static APP_NAME: &'static str = "TinyUrl 🔗 shortener CLI";
static VERSION: &'static str = "1.1";
static AUTHOR: &'static str = "Valerio Casalino <casalinovalerio.cv@gmail.com>";
static DESCRIPTION: &'static str = "CLI Rust wrapper for tinyurl's API";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // https://docs.rs/clap/2.33.0/clap/
    let matches = App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("input")
                .help("url 🔗 to shorten ✂")
                .required(true)
                .multiple(false))
        .arg(
            Arg::with_name("custom")
                .short("c")
                .long("custom")
                .multiple(false)
                .required(false)
                .takes_value(true)
                .help("Custom alias for your link 😎")
        )
        .get_matches();

    let to_shorten = matches.value_of("input").unwrap();
    let url = format!("https://tinyurl.com/api-create.php?url={}", to_shorten);
    let res = reqwest::get(url.as_str()).await?.text().await?;

    println!("{}", res);


    Ok(())
}
