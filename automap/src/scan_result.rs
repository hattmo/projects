use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Default)]
pub struct ScanResult {
    #[serde(default)]
    host: Vec<Host>,
}

#[derive(Debug, Deserialize)]
struct Host {
    address: Address,
    status: Status,
    hostnames: Option<HostNames>,
    ports: Option<Ports>,
}

#[derive(Debug, Deserialize)]
struct Address {
    #[serde(rename = "@addr")]
    addr: String,
}

#[derive(Debug, Deserialize)]
struct Status {
    #[serde(rename = "@state")]
    state: String,
}

#[derive(Debug, Deserialize)]
struct HostNames {
    #[serde(default)]
    hostname: Vec<HostName>,
}

#[derive(Debug, Deserialize)]
struct HostName {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Ports {
    #[serde(default)]
    port: Vec<Port>,
}

#[derive(Debug, Deserialize)]
struct Port {
    #[serde(rename = "@portid")]
    portid: String,
    #[serde(rename = "@protocol")]
    protocol: String,
}

impl Display for ScanResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut results = Vec::new();
        for host in &self.host {
            if host.status.state == "down" {
                continue;
            }
            let port_str = host
                .ports
                .as_ref()
                .map_or("No open ports".to_string(), |port| {
                    port.port
                        .iter()
                        .map(|p| format!("{}({})", p.protocol, p.portid))
                        .collect::<Vec<String>>()
                        .join(", ")
                });
            let hostname_str = if let Some(ref hostnames) = host.hostnames {
                if hostnames.hostname.is_empty() {
                    String::new()
                } else {
                    " (".to_string()
                        + &hostnames
                            .hostname
                            .iter()
                            .map(|h| h.name.clone())
                            .collect::<Vec<String>>()
                            .join(", ")
                        + ")"
                }
            } else {
                String::new()
            };
            results.push(format!(
                "{}{}: {}",
                host.address.addr, hostname_str, port_str
            ));
        }
        if results.is_empty() {
            results.push("No hosts found".to_string());
        }
        write!(f, "{}", results.join("\n"))?;
        Ok(())
    }
}
