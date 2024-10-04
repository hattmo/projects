use std::{
    io::{self, prelude::*},
    process::Stdio,
};

#[allow(unused)]
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

#[allow(unused)]
pub fn create_tunnel() -> io::Result<()> {
    let status = std::process::Command::new("ip")
        .args(&["link", "add", "dev", "wg0", "type", "wireguard"])
        .status()?;
    if !status.success() {
        return Err(io::Error::other("Failed to add wg dev"));
    }
    Ok(())
}
