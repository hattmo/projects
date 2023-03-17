use nix::{pty::openpty, unistd::dup};
use std::{
    ffi::OsStr, fs::File, io::prelude::*, os::fd::FromRawFd, os::unix::process::*, process::Stdio,
};

fn main() {
    let mut child = command_pty(OsStr::new(r#"ls -al"#)).unwrap();
    let mut buf = [0u8; 1024];
    let mut stdout = std::io::stdout();
    while let Ok(read) = child.reader.read(buf.as_mut()) {
        if read == 0 {
            println!("Nothing to read");
            break;
        }
        stdout.write(&buf[..read]).unwrap();
    }
}

struct ChildPty {
    pub reader: File,
    pub writer: File,
    child: std::process::Child,
}

impl Drop for ChildPty {
    fn drop(&mut self) {
        self.child.wait().unwrap();
    }
}

fn command_pty(command: &OsStr) -> Result<ChildPty, Box<dyn std::error::Error>> {
    let pty_pair = openpty(None, None)?;
    unsafe {
        let stdin = Stdio::from_raw_fd(dup(pty_pair.slave)?);
        let stdout = Stdio::from_raw_fd(dup(pty_pair.slave)?);
        let stderr = Stdio::from_raw_fd(pty_pair.slave);
        let child = std::process::Command::new(OsStr::new("/bin/bash"))
            .arg("-l")
            .arg("-i")
            .arg("-c")
            .arg(command)
            .stdin(stdin)
            .stderr(stdout)
            .stdout(stderr)
            .spawn()?;
        let writer = File::from_raw_fd(dup(pty_pair.master)?);
        let reader = File::from_raw_fd(pty_pair.master);
        Ok(ChildPty {
            reader,
            writer,
            child,
        })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {}
}
