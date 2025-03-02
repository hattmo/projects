use std::{
    ffi::{CStr, CString},
    fs::{self, read_dir, File},
    io::{self, BorrowedBuf, Error, Read, Result as IoResult},
    mem::MaybeUninit,
    net::{Ipv4Addr, SocketAddr, TcpStream, UdpSocket},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::ffi::OsStrExt,
    },
    path::Path,
    sync::RwLock,
    time::Duration,
};

use libc::{
    IN_CLOEXEC, IN_DELETE, IN_EXCL_UNLINK, IN_MODIFY, IN_MOVE, IN_MOVED_FROM, IN_MOVED_TO,
    IN_ONLYDIR,
};
use sha2::{Digest, Sha256};
use tracing::{error, info};

use crate::{FileList, Proto};

pub fn server_start() -> IoResult<()> {
    tracing_subscriber::fmt().compact().init();
    info!("Starting server");
    let cwd = std::env::current_dir()?.canonicalize()?;
    let file_list = RwLock::new(FileList::new());
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 1234))?;
    sock.set_broadcast(true)?;
    info!("Server started");
    std::thread::scope(|t| {
        t.spawn(|| listen_job(&sock, &file_list));
        t.spawn(|| watch_job(&cwd, &file_list));
    });
    Ok(())
}

#[repr(C)]
struct INotifyEvent {
    wd: u32,
    mask: u32,
    cookie: u32,
    len: u32,
    name: [u8; libc::PATH_MAX as usize],
}

fn watch_job(root: &Path, file_list: &RwLock<FileList>) -> IoResult<()> {
    let fd = unsafe { libc::inotify_init1(IN_CLOEXEC) };
    if fd == -1 {
        return Err(Error::last_os_error());
    }
    let mut instance = unsafe { File::from_raw_fd(fd) };
    let path = CString::new(root.as_os_str().as_bytes())?;
    let path_ptr = path.as_ptr();
    let res = unsafe {
        libc::inotify_add_watch(
            instance.as_raw_fd(),
            path_ptr,
            IN_DELETE | IN_MODIFY | IN_MOVE | IN_ONLYDIR | IN_EXCL_UNLINK,
        )
    };
    if res == -1 {
        return Err(Error::last_os_error());
    }
    {
        let mut lock = file_list.write().unwrap();
        *lock = read_dir(root)
            .unwrap()
            .flatten()
            .filter(|entry| {
                if let Ok(file_type) = entry.file_type() {
                    file_type.is_file()
                } else {
                    false
                }
            })
            .map(|entry| {
                let name = entry.file_name().to_str().unwrap().to_string();
                let bytes = fs::read(root.join(&name)).unwrap();
                let mut hasher = Sha256::new();
                hasher.update(&bytes);
                let hash: [u8; 32] = hasher.finalize().into();
                (name, hash)
            })
            .collect();
        info!(?lock);
    }
    let mut event: MaybeUninit<INotifyEvent> = MaybeUninit::uninit();
    loop {
        let mut buf: BorrowedBuf = event.as_bytes_mut().into();
        let cursor = buf.unfilled();
        if let Err(e) = instance.read_buf(cursor) {
            match e.kind() {
                std::io::ErrorKind::Interrupted => continue,
                _ => return Err(e),
            }
        };
        let event = unsafe { event.assume_init_ref() };
        let name = CStr::from_bytes_until_nul(&event.name)
            .unwrap()
            .to_str()
            .unwrap();
        let mut lock = file_list.write().unwrap();
        if (IN_DELETE | IN_MOVED_FROM) & event.mask != 0 {
            info!("File Removed");
            lock.remove(name);
        } else if (IN_MOVED_TO | IN_MODIFY) & event.mask != 0 {
            info!("File Added");
            let bytes = fs::read(root.join(name)).unwrap();
            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            let hash: [u8; 32] = hasher.finalize().into();
            lock.insert(name.to_string(), hash);
        } else {
            error!(event.mask, "Unknown Event");
        };
        info!(?lock);
    }
}

fn listen_job(sock: &UdpSocket, files: &RwLock<FileList>) -> IoResult<()> {
    for (mess, addr) in MessageStream(&sock) {
        match mess {
            Proto::Available => {
                info!("Got available request");
                send_file_list(&sock, files);
            }
            Proto::Transfer((name, hash)) => {
                info!("Got transfer request");
                let lock = files.read().map_err(|e| io::Error::other(e.to_string()))?;
                if lock
                    .iter()
                    .find(|(n, h)| **n == name && **h == hash)
                    .is_some()
                {
                    send_file()
                };
            }
            _ => {
                info!(%mess, "Invalid message");
            }
        }
    }
    Ok(())
}

fn send_file_list(sock: &UdpSocket, files: &RwLock<FileList>) {
    let lock = files.read().unwrap();
    let packet = Proto::FileList(lock.clone());
    let buf = bincode::serialize(&packet).unwrap();
    sock.send_to(buf.as_ref(), to_addr).unwrap();
}

struct MessageStream<'a>(&'a UdpSocket);

impl<'a> Iterator for MessageStream<'a> {
    type Item = (Proto, SocketAddr);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 1500];
        let Self(sock) = self;
        loop {
            let (read, addr) = sock.recv_from(&mut buffer).ok()?;
            let message = match bincode::deserialize(&buffer[..read]) {
                Ok(mess) => mess,
                Err(err) => {
                    error!(%err, "Error decoding message");
                    continue;
                }
            };
            return Some((message, addr));
        }
    }
}
