use std::net::IpAddr;

use chrono::{DateTime, FixedOffset};

pub enum Trigger {
    UriPath,
    UserAgent,
    Unassigned
}

pub struct BotData {
    pub name: String,
    pub ip: IpAddr,
    pub date: DateTime<FixedOffset>,
    pub uri: String,
    pub user_agent: String,
    pub triggered_on: Trigger
}
