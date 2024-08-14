use std::io::{self, prelude::*, BufReader};
use std::process::Stdio;
use std::{error::Error, fs::OpenOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let (public, private) = gen_key_pair()?;

    let mut netdev_file = OpenOptions::new()
        .write(true)
        .read(true)
        .truncate(false)
        .open("50-wg.netdev")?;
    let last_ip = BufReader::new(&mut netdev_file)
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|line| {
                if line.contains("AllowedIPs") {
                    Some(line)
                } else {
                    None
                }
            })
        })
        .last()
        .ok_or(io::Error::other("No last ip addr"))?;
    let (_, last_ip) = last_ip.split_once("=").unwrap();
    let (last_ip, _) = last_ip.split_once("/").unwrap();
    let (prefix, suffix) = last_ip.rsplit_once(".").unwrap();
    let mut suffix: u8 = suffix.parse()?;
    suffix += 1;
    let new_ip = format!("{prefix}.{suffix}");

    netdev_file.seek(io::SeekFrom::End(0))?;
    // [WireGuardPeer]
    // PublicKey=rI5gpAgEP1LohPxfeklnCF+0uNOaYTsAuWntXhgbQzg=
    // AllowedIPs=192.168.30.3/32
    writeln!(netdev_file, "")?;
    writeln!(netdev_file, "[WireGuardPeer]")?;
    writeln!(netdev_file, "PublicKey={public}")?;
    writeln!(netdev_file, "AllowedIPs={new_ip}/32")?;
    writeln!(netdev_file, "")?;
    netdev_file.flush()?;
    drop(netdev_file);

    let mut config_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("./wg_{}.conf", suffix))?;
    writeln!(config_file, "[Interface]")?;
    writeln!(config_file, "PrivateKey = {private}")?;
    writeln!(config_file, "Address = {new_ip}/24")?;
    writeln!(config_file, "")?;
    writeln!(config_file, "[Peer]")?;
    writeln!(
        config_file,
        "PublicKey = k79Sxvn7IZC+kRiw5k3a9Gijv1jE0LUFgP1TuT9Quwg="
    )?;
    writeln!(
        config_file,
        "AllowedIPs = 192.168.30.0/24, 192.168.20.0/24, 192.168.10.0/24"
    )?;
    writeln!(config_file, "Endpoint = 172.104.208.95:42069")?;
    Ok(())
}

fn gen_key_pair() -> Result<(String, String), io::Error> {
    let output = std::process::Command::new("wg").arg("genkey").output()?;
    let private = output.stdout;
    let mut pub_key_proc = std::process::Command::new("wg")
        .arg("pubkey")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let Some(stdin) = &mut pub_key_proc.stdin else {
        return Err(io::Error::other("Failed to gen pubkey"));
    };
    stdin.write_all(&private)?;
    stdin.flush()?;
    let status = pub_key_proc.wait()?;
    if !status.success() {
        return Err(io::Error::other("Failed to gen pubkey"));
    }
    let mut stdout = pub_key_proc
        .stdout
        .take()
        .ok_or(io::Error::other("No stdout"))?;
    let mut public = Vec::new();
    stdout.read_to_end(&mut public)?;
    let private = String::from_utf8(private)
        .or(Err(io::Error::other("Failed to parse")))?
        .trim()
        .to_owned();
    let public = String::from_utf8(public)
        .or(Err(io::Error::other("Failed to parse")))?
        .trim()
        .to_owned();
    Ok((public, private))
}
