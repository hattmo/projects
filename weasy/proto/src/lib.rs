use std::{io::SeekFrom, net::SocketAddr, path::PathBuf, time::Duration};

pub mod response {
    use bincode::{Decode, Encode};
    #[derive(Encode, Decode)]
    pub struct Response {
        pub session: u64,
        pub seq: u64,
        pub res: ResponseType,
    }
    #[derive(Encode, Decode)]
    pub enum ResponseType {
        NewHandle(NewHandle),
        HandleNotFound(u64),
        Error(String),
        Read(Read),
    }

    #[derive(Encode, Decode)]
    pub struct Read {
        pub data: Vec<u8>,
    }

    #[derive(Encode, Decode)]
    pub struct NewHandle {
        pub note: String,
        pub id: u64,
    }
}
pub mod request {
    use bincode::{Decode, Encode};
    use std::time::Duration;

    use std::net::SocketAddr;

    use std::io::SeekFrom;

    use std::path::PathBuf;

    #[derive(Encode, Decode)]
    pub struct Request {
        pub session: u64,
        pub seq: u64,
        pub req: RequestType,
    }

    #[derive(Encode, Decode)]
    pub enum RequestType {
        Exec(Exec),
        Open(Open),
        Close(Close),
        Read(Read),
        Write(Write),
        Seek(Seek),
        Bind(Bind),
        UnBind(UnBind),
        Connect(Connect),
        Shutdown,
        Config(Config),
    }

    #[derive(Encode, Decode)]
    pub struct Exec {
        pub command: String,
        pub args: Vec<String>,
        pub env: Vec<Env>,
        pub current_dir: Option<PathBuf>,
        pub uid: Option<u32>,
        pub gid: Option<u32>,
    }

    #[derive(Encode, Decode)]
    pub enum Env {
        Clear,
        Delete(String),
        Add((String, String)),
        Append((String, String)),
    }

    #[derive(Encode, Decode)]
    pub struct Open {
        pub path: PathBuf,
    }

    #[derive(Encode, Decode)]
    pub struct Close {
        pub id: u64,
    }

    #[derive(Encode, Decode)]
    pub struct Read {
        pub id: u64,
        pub ammount: ReadAmount,
    }

    #[derive(Encode, Decode)]
    pub enum ReadAmount {
        End,
        Exact(usize),
        Some(usize),
    }

    #[derive(Encode, Decode)]
    pub struct Write {
        pub id: u64,
        pub all: bool,
        pub content: Vec<u8>,
    }

    #[derive(Encode, Decode)]
    pub struct Seek {
        pub id: u64,
        pub wence: Wence,
    }

    #[derive(Encode, Decode)]
    pub enum Wence {
        Start(u64),
        End(i64),
        Current(i64),
    }

    impl Into<SeekFrom> for Wence {
        fn into(self) -> SeekFrom {
            match self {
                Wence::Start(i) => SeekFrom::Start(i),
                Wence::End(i) => SeekFrom::End(i),
                Wence::Current(i) => SeekFrom::Current(i),
            }
        }
    }

    #[derive(Encode, Decode)]
    pub struct Bind {
        pub addr: SocketAddr,
    }

    #[derive(Encode, Decode)]
    pub struct UnBind {
        pub id: u64,
    }

    #[derive(Encode, Decode)]
    pub struct Connect {
        pub addr: SocketAddr,
    }

    #[derive(Encode, Decode)]
    pub struct Config {
        pub sleep: Duration,
    }

    #[cfg(test)]
    pub(crate) mod test {
        use std::io::Read;

        #[test]
        pub fn playground() -> std::io::Result<()> {
            let mut f = std::fs::File::open("/etc/shadow")?;
            let mut out = String::new();
            f.read_to_string(&mut out)?;
            println!("{out}");
            Ok(())
        }
    }
}
