use crate::Trigger;
use crate::BotData;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr};
use std::str::SplitWhitespace;

pub fn parse_log(input: &str) -> Vec<BotData> {
    let file = match File::open(input) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to open input file: {}", e);
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);

    let mut founds: Vec<BotData> = vec!();
    for (_, line) in reader.lines().enumerate() {
        let mut found: BotData = BotData{
            name: String::new(),
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            uri: String::new(),
            user_agent: String::new(),
            triggered_on: Trigger::Unassigned
        };

        let line = line.unwrap();
        //println!("{}", line);
        let mut split_line: SplitWhitespace = line.as_str().split_whitespace();

        let ip_addr = split_line.next().unwrap();
        if ip_addr.contains(":") {
            found.ip = IpAddr::V6(ip_addr.parse().unwrap());
        } else { 
            found.ip = IpAddr::V4(ip_addr.parse().unwrap());
        }

        let mut uri_split = split_line.skip(5);
        let uri = uri_split.next().unwrap().to_string();
        found.uri = uri.clone();
        match crate::regexes::bot_uris(uri) {
            Some(s) => {
                found.triggered_on = Trigger::UriPath;
                found.name = s.to_string();
            },
            None => ()
        }

        let ua_split = uri_split.skip(4);
        for (_, ua_part) in ua_split.enumerate() {
            found.user_agent.push_str(format!("{} ", ua_part).replace("\"", "").trim());
        }

        match crate::regexes::bot_uas(&found.user_agent) {
            Some(s) => {
                found.triggered_on = Trigger::UserAgent;
                found.name = s.to_string();
            },
            None => ()
        }

        match found.triggered_on {
            Trigger::Unassigned => (),
            _ => founds.push(found)
        }
    }

    return founds;
}
