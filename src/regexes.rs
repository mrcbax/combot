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
            return Some("ZoomInfo".to_string());
        }
        if value.contains("GuzzleHttp") {
            return Some("guzzle".to_string());
        }
        if value.contains("curl") {
            return Some("curl".to_string());
        }
        if value.contains("Elisa") {
            return Some("elisabot".to_string());
        }
        if value.contains("Censys") {
            return Some("censys".to_string());
        }
        if value.contains("Dispatch") {
            return Some("java-dispatch".to_string());
        }
        if value.contains("Barkowler") {
            return Some("barkowler".to_string());
        }
        if value.contains("LightspeedSystems") {
            return Some("lightspeed".to_string());
        }
        if value.contains("Headless") {
            return Some("generic".to_string());
        }
        if value.contains("Adsbot") {
            return Some("adsbot".to_string());
        }
        if value.contains("adstxt") {
            return Some("advertisment-fraud".to_string());
        }
        if value.contains("Blackboard") {
            return Some("blackboard".to_string());
        }
        if value.contains("crawler4j") {
            return Some("java-crawler4j".to_string());
        }
        if value.contains("Apache-HttpClient") {
            return Some("apache".to_string());
        }
        if value.contains("Pleroma") {
            return Some("pleroma".to_string());
        }
        if value.contains("Expanse") {
            return Some("expanseinc".to_string());
        }
        /*if value.contains("rb") {
            return Some("ruby-http".to_string());
        }*/
        if value.contains("Synapse") {
            return Some("synapse".to_string());
        }
        if value.contains("MTRobot") {
            return Some("mtrobot".to_string());
        }
        if value.contains("Go http package") {
            return Some("go-http".to_string());
        }
        if value.contains("got/9") {
            return Some("got".to_string());
        }
        if value.contains("NetSystems") {
            return Some("netsystemsresearch".to_string());
        }
        if value.contains("Test Certificate Info") {
            return Some("windowscpp-certificateprobe".to_string());
        }
        if value.contains("quic-go-HTTP") {
            return Some("go-http".to_string());
        }
        if value.contains("TprAdsTxtCrawler") {
            return Some("advertisment-fraud".to_string());
        }
        if value.contains("Mojolicious") {
            return Some("perl-mojolicious".to_string());
        }
        if value.contains("lkxscan") || value.contains("l9tcpid") {
            return Some("secrets-leakix".to_string());
        }
        if value.contains("onsiteexplorer") || value.contains("Semrush") {
            return Some("marketing-mining".to_string());
        }
        if value.contains("MojeekBot") {
            return Some("mojeek".to_string());
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
        if value.contains("wp-content") || value.contains("wp-includes") || value.contains("wp-login") || value.contains("wp-admin") {
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
        if value.contains("humans.txt") {
            return Some("harvest-pii".to_string());
        }
        if value.contains("phpstorm") || value.contains("ThinkPHP") {
            return Some("exploit-thinkphp".to_string());
        }
        if value.contains("mstshash=Administr") {
            return Some("exploit-msts".to_string());
        }
        if value.contains(".vscode") || value.contains(".ftpconfig") || value.contains("deployment-config.json") || value.contains("sftp-config.json") || value.contains("ftpsync.settings") {
            return Some("secrets-vscode".to_string());
        }
        if value.contains("microsoft.exchange.ediscovery.exporttool.application") {
            return Some("exploit-msexchange".to_string());
        }
        if value.contains("magento") || value.contains("staging") || value.contains("downloader") {
            return Some("exploit-magento".to_string());
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
