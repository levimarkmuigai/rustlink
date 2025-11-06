use std::net::{IpAddr, ToSocketAddrs};
use url::Url as ExternalUrl;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    raw: String,
}

impl Url {
    pub fn new(value: &str) -> Result<Self, String> {
        let value_trim = value.trim();

        if value_trim.is_empty() {
            return Err("Empty url".to_string());
        }

        // OWASP 08 Data integrity faliure
        let parsed =
            ExternalUrl::parse(value_trim).map_err(|_| "Invalid URL format".to_string())?;

        let domain = parsed
            .host_str()
            .ok_or_else(|| "Missing domain in URL".to_string())?;

        // OWASP 10 Server Side Request  Forgery
        let socket_addrs = (domain, 25)
            .to_socket_addrs()
            .map_err(|_| "Cannot resolve domain")?;

        if socket_addrs
            .map(|addr| addr.ip())
            .any(|ip| is_private_ip(&ip))
        {
            return Err("Domain resolves to a private Ip".to_string());
        }

        Ok(Self {
            raw: value.to_string(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

// OWASP 10 Server Side Request Forgery
fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => ipv4.is_private() || ipv4.is_loopback(),
        IpAddr::V6(ipv6) => ipv6.is_loopback() || ipv6.is_unique_local(),
    }
}
