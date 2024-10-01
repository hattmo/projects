#![feature(exit_status_error)]
use std::{
    io,
    process::{Command, Output},
};

fn main() {
    match commands() {
        Ok(_) => println!("[+] Done"),
        Err(err) => {
            println!("[!] Failed: {err}")
        }
    }
}
fn commands() -> io::Result<()> {
    run(&["git", "fetch", "--all"])?;
    let (branches, _) = run(&["git", "branch", "-a"])?;
    let bot_branches: Vec<_> = branches
        .lines()
        .filter_map(|line| {
            if line.contains("origin/dependabot") {
                line.split("/").nth(2)
            } else {
                None
            }
        })
        .collect();
    if bot_branches.is_empty() {
        println!("[!] No dependa bot branches");
        return Ok(());
    }
    for branch in bot_branches {
        run(&["git", "switch", branch])?;
        run(&["git", "rebase", "main"])?;
        run(&["git", "push", "--force"])?;
        run(&["git", "switch", "main"])?;
        run(&["git", "merge", branch])?;
        run(&["git", "push", "-d", "origin", branch])?;
        run(&["git", "branch", "-d", branch])?;
    }
    run(&["git", "rebase", "-i", "origin/main"])?;
    Ok(())
}

fn run(command: &[&str]) -> io::Result<(String, String)> {
    let command_str = command.join(" ");
    println!("[+] {command_str}");
    let [command, args @ ..] = command else {
        return Err(io::Error::other("Invalid command"));
    };
    let Output {
        status,
        stdout,
        stderr,
    } = Command::new(command).args(args).output()?;
    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);
    if !stdout.is_empty() {
        println!("====STDOUT====\n{stdout}\n==============");
    }
    if !stderr.is_empty() {
        println!("====STDERR====\n{stderr}\n==============");
    }

    let code = status.code();
    match code {
        Some(code) if code != 0 => {
            println!("[!] Return: {code}");
            return Err(io::Error::other("Command returned non-zero code"));
        }
        None => println!("Unknown return"),
        _ => {}
    }
    Ok((stdout.to_string(), stderr.to_string()))
}
