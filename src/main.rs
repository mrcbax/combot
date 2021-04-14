extern crate clap;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use clap::{crate_version, crate_name, crate_description, App, Arg};

mod nginx_parser;

enum Trigger {
    UriPath,
    UserAgent
}

struct BotData {
    name: String,
    ip: IpAddr,
    user_agent: String,
    triggered_on: Trigger
}

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
    // use crate_version! to pull the version number
        .version(crate_version!())
        .arg(
            Arg::with_name("input_format")
                .help("Select the output format.")
                .short("-i")
                .long("--input_format")
                .takes_value(true)
                .multiple(false)
                .required(true)
                .requires("input")
        )
        .arg(
            Arg::with_name("output_format")
                .help("Select the output format.")
                .short("-f")
                .long("--output_format")
                .takes_value(true)
                .multiple(false)
                .required(true)
                .requires("output")
        )
        .arg(
            Arg::with_name("input")
                .help("the input file to use.")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("the output file to use.")
                .required(true),
        )
        .get_matches();
    match matches.value_of("input_format").unwrap() {
        "nginx" => nginx_parser::parse_log(
            matches.value_of("input").unwrap(),
            matches.value_of("output").unwrap(),
            matches.value_of("output_format").unwrap()
        ),
        _ => todo!()
    }
}
