use clap::{crate_version, crate_name, crate_description, crate_authors, App, Arg};

pub mod parsers;
pub mod regexes;
pub mod output;
pub mod types;

use types::*;

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(format!("{}\n{}", crate_version!(), "GNU-GPL-3.0").as_str())
        .arg(
            Arg::with_name("input_format")
                .help("Select the output format: nginx")
                .short("-i")
                .long("--input_format")
                .takes_value(true)
                .multiple(false)
                .required(true)
                .requires("input")
        )
        .arg(
            Arg::with_name("output_format")
                .help("Select the output format: csv, abuseipdb-csv")
                .short("-f")
                .long("--output_format")
                .takes_value(true)
                .multiple(false)
                .required(true)
                .requires("output")
        )
        .arg(
            Arg::with_name("uri_list")
                .help("Specify a path to a list of URI pieces to trigger on.")
                .short("-u")
                .long("--uri_list")
                .takes_value(true)
                .multiple(false)
                .required(false)
        )
        .arg(
            Arg::with_name("ua_list")
                .help("Specify a path to a list of User Agent pieces to trigger on.")
                .short("-a")
                .long("--ua_list")
                .takes_value(true)
                .multiple(false)
                .required(false)
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

    let mut uri_path = "";
    let mut ua_path = "";
    if matches.is_present("uri_list") {
        uri_path = matches.value_of("uri_list").unwrap();
    }
    if matches.is_present("ua_list") {
        ua_path = matches.value_of("ua_list").unwrap();
    }

    let founds: Vec<BotData> = match matches.value_of("input_format").unwrap() {
        "nginx" => parsers::nginx::parse(
            matches.value_of("input").unwrap(),
            uri_path,
            ua_path
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
            "json" => eprintln!("JSON output not yet implemented."),
            _ => eprintln!("Specified output format does not exist.")
        }
    }
}
