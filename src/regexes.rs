use regex::Regex;

pub fn bot_uas<'a>(value: &String) -> Option<&'a str> {
    if value.contains("zgrab") {
        return Some("zgrab");
    }
    if value.contains("python") {
        return Some("python-requests");
    }
    if value.contains("ZoominfoBot") {
        Some("ZoomInfo");
    }
    if value.contains("GuzzleHttp") {
        Some("guzzle");
    }
    if value.contains("curl") {
        Some("curl");
    }
    if value.contains("Elisa") {
        Some("elisabot");
    }
    if value.contains("Censys") {
        Some("censys");
    }
    if value.contains("Dispatch") {
        Some("java-dispatch");
    }
    if value.contains("Barkowler") {
        Some("barkowler");
    }
    if value.contains("Semrush") {
        Some("semrush");
    }
    if value.contains("LightspeedSystems") {
        Some("lightspeed");
    }
    if value.contains("Headless") {
        Some("generic");
    }
    if value.contains("Adsbot") {
        Some("adsbot");
    }
    if value.contains("adstxt") {
        Some("adstxt");
    }
    if value.contains("Blackboard") {
        Some("blackboard");
    }
    if value.contains("crawler4j") {
        Some("java-crawler4j");
    }
    if value.contains("Apache-HttpClient") {
        Some("apache");
    }
    if value.contains("Pleroma") {
        Some("pleroma");
    }
    if value.contains("Expanse") {
        Some("expanseinc");
    }
    if value.contains("rb") {
        Some("ruby-http");
    }
    if value.contains("Synapse") {
        Some("synapse");
    }
    if value.contains("MTRobot") {
        Some("mtrobot");
    }
    if value.contains("DotBot") {
        Some("dotbot");
    }
    if value.contains("Go http package") {
        Some("go-http");
    }
    if value.contains("got/9") {
        Some("got");
    }
    if value.contains("NetSystems") {
        Some("netsystemsresearch");
    }
    return None;
}

pub fn bot_uris<'a>(value: String) -> Option<&'a str> {
    if value.contains("wp-content") || value.contains("wp-includes") {
        return Some("exploit-wordpress")
    }
    if value.contains("phpunit") {
        return Some("exploit-phpunit");
    }
    if value.contains(".git") || value.contains(".svn") {
        return Some("secrets-versioncontrol");
    }
    if value.contains("nice%20ports") || value.contains("nmap") {
        return Some("nmap");
    }
    if value.contains("vendor") {
        return Some("exploit-php");
    }
    if value.contains("laravel") {
        return Some("exploit-laravel");
    }
    if value.contains("dns-query") {
        return Some("exploit-nginx");
    }
    return None;
}
