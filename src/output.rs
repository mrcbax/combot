use crate::types::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use csv::WriterBuilder;

pub fn csv(output_file: &str, founds: Vec<BotData>) {
    let file = match File::create(output_file) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            std::process::exit(1);
        }
    };
    let mut writer = LineWriter::new(file);
    writer.write_all(b"name,ip,date,user_agent,triggered_on\n").unwrap();
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
            found.date.to_rfc3339().as_str(),
            found.user_agent.as_str(),
            trigger
        ]).unwrap();
        writer.write_all(csv_writer.into_inner().unwrap().as_slice()).unwrap();
    }
    writer.flush().unwrap();
}

pub fn abuseipdb_csv(output_file: &str, founds: Vec<BotData>) {
    let file = match File::create(output_file) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            std::process::exit(1);
        }
    };
    let mut writer = LineWriter::new(file);
    writer.write_all(b"IP,Categories,ReportDate,Comment\n").unwrap();
    for mut found in founds {
        found.uri = found.uri.replace('"', "");
        found.user_agent = found.user_agent.replace('"', "");
        let trigger: &str = match found.triggered_on {
            Trigger::UriPath => "uri_path",
            Trigger::UserAgent => "user_agent",
            Trigger::Unassigned => "none"
        };
        let mut csv_writer = WriterBuilder::new()
            .quote_style(csv::QuoteStyle::Never)
            .from_writer(vec![]);
        csv_writer.write_record(&[
            found.ip.to_string().as_str(),
            "19",
            found.date.to_rfc3339().as_str(),
            match found.triggered_on {
                Trigger::UriPath => format!("\"{} - {}: {}\"", found.name, trigger, found.uri),
                Trigger::UserAgent => format!("\"{} - {}: {}\"", found.name, trigger, found.user_agent),
                _ => format!("\"{} - {}\"", found.name, trigger),
            }.as_str()
        ]).unwrap();
        writer.write_all(csv_writer.into_inner().unwrap().as_slice()).unwrap();
    }
    writer.flush().unwrap();
}
