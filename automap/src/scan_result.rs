use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScanResult {
    host: Option<Vec<Host>>,
}

#[derive(Debug, Deserialize)]
struct Host {
    address: Address,
    ports: Ports,
}
#[derive(Debug, Deserialize)]
struct Address {
    #[serde(rename = "@addr")]
    addr: String,
    #[serde(rename = "@addrtype")]
    addrtype: String,
}
#[derive(Debug, Deserialize)]
struct Ports {
    port: Option<Vec<Port>>,
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
        if let Some(host) = &self.host {
            for host in host {
                if let Some(ports) = &host.ports.port {
                    let ports = ports
                        .iter()
                        .map(|p| p.portid.clone())
                        .collect::<Vec<String>>()
                        .join(",");
                    results.push(format!("{}: {}", host.address.addr, ports));
                }
            }
        }
        write!(f, "{}", results.join("\n"))?;
        Ok(())
    }
}

impl Default for ScanResult {
    fn default() -> Self {
        Self { host: None }
    }
}
