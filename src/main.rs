extern crate clap;
extern crate csv;
extern crate regex;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use clap::{crate_version, crate_name, crate_description, App, Arg};
use csv::WriterBuilder;

pub mod nginx_parser;
pub mod regexes;

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
        "nginx" => nginx_parser::parse_log(
            matches.value_of("input").unwrap(),
        ),
        _ => todo!()
    };
    if founds.len() > 0 {
        match matches.value_of("output_format").unwrap() {
            "csv" => {
                let file = match File::create(matches.value_of("output").unwrap()) {
                    Ok(o) => o,
                    Err(e) => {
                        eprintln!("Failed to create output file: {}", e);
                        std::process::exit(1);
                    }
                };
                let mut writer = LineWriter::new(file);
                writer.write_all(b"name,ip,user_agent,triggered_on\n").unwrap();
                for found in founds {
                    let trigger: &str = match found.triggered_on {
                        Trigger::UriPath => "uri_path",
                        Trigger::UserAgent => "user_agent",
                        Trigger::Unassigned => "none"
                    };
                    let mut csv_writer = WriterBuilder::new().from_writer(vec![]);
                    csv_writer.write_record(&[
                        found.name.as_str(),
                        found.ip.to_string().as_str(),
                        found.user_agent.as_str(), trigger
                    ]).unwrap();
                    writer.write_all(csv_writer.into_inner().unwrap().as_slice()).unwrap();
                }
                writer.flush().unwrap();
            },
            "json" => todo!(),
            _ => todo!()
        }
    }
}
