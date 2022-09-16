use ssh2::Session;
use std::{
    collections::HashMap,
    io::{self, prelude::*},
    net::TcpStream,
    path::Path,
};
pub trait Machine {
    fn conn_info(&self) -> &ConnInfo;
    fn name(&self) -> &str;
}

pub enum Cred<'a> {
    Password(String),
    PubKey(KeyCreds<'a>),
}
pub struct KeyCreds<'a> {
    privkey: &'a Path,
    pubkey: Option<&'a Path>,
    passphrase: Option<&'a str>,
}

pub struct ConnInfo<'a> {
    pub host: &'a str,
    pub port: u16,
    pub user: &'a str,
    pub creds: Cred<'a>,
}

pub struct Environment {
    handles: HashMap<String, MachineHandle>,
    #[allow(dead_code)]
    machines: Vec<Box<dyn Machine>>,
}

impl Environment {
    pub fn new(
        machines: Vec<Box<dyn Machine>>,
        provisioner: fn(sessions: &HashMap<String, MachineHandle>) -> (),
    ) -> Self {
        let handles = machines
            .iter()
            .map(|machine| {
                let conn_info = machine.conn_info();
                let mut session = Session::new().unwrap();
                session
                    .set_tcp_stream(TcpStream::connect((conn_info.host, conn_info.port)).unwrap());
                session.handshake().unwrap();
                match conn_info.creds {
                    Cred::Password(ref password) => {
                        session
                            .userauth_password(&conn_info.user, &password)
                            .unwrap();
                    }
                    Cred::PubKey(ref key) => {
                        session
                            .userauth_pubkey_file(
                                &conn_info.user,
                                key.pubkey,
                                &key.privkey,
                                key.passphrase,
                            )
                            .unwrap();
                    }
                }
                (
                    machine.name().to_owned(),
                    MachineHandle {
                        session,
                        host: conn_info.host.to_owned(),
                    },
                )
            })
            .collect();
        provisioner(&handles);
        Self {
            machines,
            handles: handles,
        }
    }

    pub fn get_handle(&self, name: &str) -> &MachineHandle {
        self.handles.get(name).unwrap()
    }
}

pub struct MachineHandle {
    session: Session,
    host: String,
}

impl MachineHandle {
    pub fn exec(&self, command: &str) -> io::Result<String> {
        self.session.set_timeout(100);
        let mut channel = self.session.channel_session()?;
        let mut s = String::new();
        channel.exec(command)?;
        channel.read_to_string(&mut s).unwrap();
        Ok(s)
    }

    pub fn host(&self) -> &str {
        &self.host
    }
}
