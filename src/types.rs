use std::net::IpAddr;

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
