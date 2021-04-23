use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn bot_uas(value: &String, ua_path: &str) -> Option<String> {
    if ua_path.len() == 0 {
        if value.contains("zgrab") {
            return Some("zgrab".to_string());
        }
        if value.contains("python") {
            return Some("python-requests".to_string());
        }
        if value.contains("ZoominfoBot") {
            Some("ZoomInfo".to_string());
        }
        if value.contains("GuzzleHttp") {
            Some("guzzle".to_string());
        }
        if value.contains("curl") {
            Some("curl".to_string());
        }
        if value.contains("Elisa") {
            Some("elisabot".to_string());
        }
        if value.contains("Censys") {
            Some("censys".to_string());
        }
        if value.contains("Dispatch") {
            Some("java-dispatch".to_string());
        }
        if value.contains("Barkowler") {
            Some("barkowler".to_string());
        }
        if value.contains("Semrush") {
            Some("semrush".to_string());
        }
        if value.contains("LightspeedSystems") {
            Some("lightspeed".to_string());
        }
        if value.contains("Headless") {
            Some("generic".to_string());
        }
        if value.contains("Adsbot") {
            Some("adsbot".to_string());
        }
        if value.contains("adstxt") {
            Some("adstxt".to_string());
        }
        if value.contains("Blackboard") {
            Some("blackboard".to_string());
        }
        if value.contains("crawler4j") {
            Some("java-crawler4j".to_string());
        }
        if value.contains("Apache-HttpClient") {
            Some("apache".to_string());
        }
        if value.contains("Pleroma") {
            Some("pleroma".to_string());
        }
        if value.contains("Expanse") {
            Some("expanseinc".to_string());
        }
        if value.contains("rb") {
            Some("ruby-http".to_string());
        }
        if value.contains("Synapse") {
            Some("synapse".to_string());
        }
        if value.contains("MTRobot") {
            Some("mtrobot".to_string());
        }
        if value.contains("DotBot") {
            Some("dotbot".to_string());
        }
        if value.contains("Go http package") {
            Some("go-http".to_string());
        }
        if value.contains("got/9") {
            Some("got".to_string());
        }
        if value.contains("NetSystems") {
            Some("netsystemsresearch".to_string());
        }
        if value.contains("Test Certificate Info") {
            Some("windowscpp-certificateprobe".to_string());
        }
        return None;
    } else {
        let file = match File::open(ua_path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Failed to open User Agent list file: {}", e);
                std::process::exit(1);
            }
        };
        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut ua_parts = line.split("|");
            let name = ua_parts.next().unwrap();
            let ua_pattern: String  = ua_parts.collect();
            if value.contains(ua_pattern.as_str()) {
                return Some(name.to_string());
            }
        }
    }
    return None
}

pub fn bot_uris(value: String, uri_path: &str) -> Option<String> {
    if uri_path.len() == 0 {
        if value.contains("wp-content") || value.contains("wp-includes") {
            return Some("exploit-wordpress".to_string())
        }
        if value.contains("phpunit") {
            return Some("exploit-phpunit".to_string());
        }
        if value.contains(".git") || value.contains(".svn") {
            return Some("secrets-versioncontrol".to_string());
        }
        if value.contains("nice%20ports") || value.contains("nmap") {
            return Some("nmap".to_string());
        }
        if value.contains("vendor") {
            return Some("exploit-php".to_string());
        }
        if value.contains("laravel") {
            return Some("exploit-laravel".to_string());
        }
        if value.contains("dns-query") {
            return Some("exploit-nginx".to_string());
        }
        if value.contains("xmlrpc") {
            return Some("exploit-wordpress".to_string());
        }
        return None;
    } else {
        let file = match File::open(uri_path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Failed to open URI list file: {}", e);
                std::process::exit(1);
            }
        };
        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut uri_parts = line.split("|");
            let name = uri_parts.next().unwrap();
            let uri_pattern: String  = uri_parts.collect();
            if value.contains(uri_pattern.as_str()) {
                return Some(name.to_string());
            }
        }
    }
    return None;
}
