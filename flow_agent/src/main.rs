#![feature(ascii_char)]
use std::{collections::HashMap, error::Error};

use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
//use serde_json::Value;

#[derive(Deserialize)]
struct Vm {
    //memory_size_MiB: usize,
    vm: String,
    name: String,
    //poser_state: String,
    //cpu_count: usize,
}

#[derive(Deserialize)]
struct VmResponse {
    value: Vec<Vm>,
}

#[derive(Deserialize)]
struct SessionResponse {
    value: String,
}

async fn login(
    client: &Client,
    host: &str,
    user: &str,
    pass: &str,
) -> Result<String, Box<dyn Error>> {
    let text = client
        .post(format!("https://{host}/rest/com/vmware/cis/session"))
        .basic_auth(user, Some(pass))
        .send()
        .await?
        .text()
        .await?;
    let val: SessionResponse = serde_json::from_str(&text)?;
    Ok(val.value)
}

async fn get_vms(
    client: &Client,
    session: &str,
    host: &str,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let text = client
        .get(format!("https://{host}/rest/vcenter/vm"))
        .header("vmware-api-session-id", session)
        .send()
        .await?
        .text()
        .await?;
    let res: VmResponse = serde_json::from_str(&text)?;
    Ok(res
        .value
        .into_iter()
        .map(|item| (item.name, item.vm))
        .collect())
}

async fn send_keys(
    client: &Client,
    session: &str,
    host: &str,
    vm_id: &str,
    keys: &str,
) -> Result<(), Box<dyn Error>> {
    let key_codes: Box<[String]> = keys
        .chars()
        .into_iter()
        .filter_map(|c| c.as_ascii())
        .map(|c| c.to_u8().to_string())
        .collect();
    let key_codes_len = key_codes.len();
    let key_codes = key_codes.join(" ");
    let body = format!(
        include_str!("./envelope.txt"),
        vm_id, key_codes, key_codes_len
    );
    println!("{body}");
    let res = client
        .post(format!("https://{host}/sdk"))
        .header("Content-Type", "text/xml")
        .header("SOAPAction", "urn:vim25/6.5")
        .header("Cookie", format!("vmware-api-session-id={session}"))
        .body(body)
        .send()
        .await?;
    let bytes = res.bytes().await?;
    let content = String::from_utf8_lossy(bytes.as_ref());
    println!("response:");
    println!("{content}");
    Ok(())
}

#[derive(clap::Parser)]
struct Args {
    username: String,
    password: String,
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let session = login(&client, &args.host, &args.username, &args.password).await?;
    let vms = get_vms(&client, &session, &args.host).await?;
    let test_vm_id = vms.get("monitor").unwrap();
    send_keys(&client, &session, &args.host, &test_vm_id, "echo foo\x1C").await?;
    Ok(())
}
