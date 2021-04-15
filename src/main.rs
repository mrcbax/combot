extern crate clap;
extern crate csv;
extern crate regex;

use std::net::IpAddr;

use clap::{crate_version, crate_name, crate_description, App, Arg};

pub mod parsers;
pub mod regexes;
pub mod output;

pub enum Trigger {
    UriPath,
    UserAgent,
    Unassigned
}

pub struct BotData {
    pub name: String,
    pub ip: IpAddr,
    pub uri: String,
    pub user_agent: String,
    pub triggered_on: Trigger
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
    let founds: Vec<BotData> = match matches.value_of("input_format").unwrap() {
        "nginx" => parsers::nginx::parse(
            matches.value_of("input").unwrap(),
        ),
        _ => todo!()
    };
    if founds.len() > 0 {
        match matches.value_of("output_format").unwrap() {
            "abuseipdb-csv" => {
                output::abuseipdb_csv(matches.value_of("output").unwrap(), founds);
            },
            "csv" => {
                output::csv(matches.value_of("output").unwrap(), founds);
            },
            "json" => todo!(),
            _ => todo!()
        }
    }
}
