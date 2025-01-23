use std::{
    ffi::CString,
    fs::File,
    io::{BorrowedBuf, Error, Read, Result as IoResult},
    mem::MaybeUninit,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::ffi::OsStrExt,
    },
    sync::{
        mpsc::{Receiver, Sender},
        Arc, RwLock,
    },
};

use libc::{IN_CLOEXEC, IN_CREATE, IN_DELETE, IN_EXCL_UNLINK, IN_MOVE, IN_ONLYDIR};
use sha2::{Digest, Sha256};
use tracing::{error, info};

use crate::{FileChunk, FileEntry, Proto};

pub fn server_start() -> IoResult<()> {
    tracing_subscriber::fmt().compact().init();
    info!("Starting server");
    let file_list = RwLock::new(Vec::new().into());
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 1234))?;
    sock.set_broadcast(true)?;
    info!("Server started");
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::scope(|t| {
        [
            t.spawn(|| listen_job(&sock, tx, &file_list)),
            t.spawn(|| watch_job(&file_list)),
            t.spawn(|| send_job(&sock, rx)),
        ]
        .into_iter()
        .for_each(|h| {
            h.join().unwrap().unwrap();
        });
    });
    Ok(())
}

fn send_job(sock: &UdpSocket, rx: Receiver<(FileEntry, SocketAddr)>) -> IoResult<()> {
    let mut buffer = [0u8; 1000];
    for (req, to) in rx {
        let cwd = std::env::current_dir()?.canonicalize()?.join(&req.name);
        let mut file = File::open(cwd)?;
        for i in 0.. {
            let Ok(read) = file.read(&mut buffer[..]) else {
                break;
            };
            if read == 0 {
                break;
            }
            let chunk = FileChunk {
                name: req.name.clone(),
                chunk: i,
                content: (&buffer[0..read]).to_vec(),
            };
            let encoded = bincode::serialize(&chunk).unwrap();
            sock.send_to(&encoded, to).unwrap();
        }
    }
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

fn watch_job(file_list: &RwLock<Arc<[FileEntry]>>) -> IoResult<()> {
    let cwd = std::env::current_dir()?.canonicalize()?;

    let fd = unsafe { libc::inotify_init1(IN_CLOEXEC) };
    if fd == -1 {
        return Err(Error::last_os_error());
    }
    let mut instance = unsafe { File::from_raw_fd(fd) };
    let path = CString::new(cwd.as_os_str().as_bytes())?;
    let path_ptr = path.as_ptr();
    let res = unsafe {
        libc::inotify_add_watch(
            instance.as_raw_fd(),
            path_ptr,
            IN_CREATE | IN_DELETE | IN_MOVE | IN_ONLYDIR | IN_EXCL_UNLINK,
        )
    };
    if res == -1 {
        return Err(Error::last_os_error());
    }
    let mut event: MaybeUninit<INotifyEvent> = MaybeUninit::uninit();
    loop {
        let mut buf: BorrowedBuf = event.as_bytes_mut().into();
        let cursor = buf.unfilled();
        if let Err(e) = instance.read_buf(cursor) {
            match e.kind() {
                std::io::ErrorKind::Interrupted => continue,
                _ => break,
            }
        };
        let mut lock = file_list.write().unwrap();
        *lock = std::fs::read_dir(&cwd)?
            .flat_map(|entry| entry.ok())
            .filter_map(|i| i.file_type().ok()?.is_file().then(|| i.path()))
            .filter_map(|path| {
                let content = std::fs::read(&path).ok()?;
                let mut hasher = Sha256::new();
                hasher.update(content.as_slice());
                let hash: Vec<u8> = hasher.finalize().to_vec();
                let size = content.len() as u128;
                let name = path.file_name()?.to_str()?.to_owned();
                Some(FileEntry { hash, size, name })
            })
            .collect::<Arc<_>>();
        println!("{lock:?}");
    }
    Ok(())
}

fn listen_job(
    sock: &UdpSocket,
    tx: Sender<(FileEntry, SocketAddr)>,
    files: &RwLock<Arc<[FileEntry]>>,
) -> IoResult<()> {
    for (mess, addr) in MessageStream(&sock) {
        match mess {
            Proto::Available => {
                info!("Got available request");
                send_file_list(&sock, files, addr);
            }
            Proto::Transfer(transfer) => {
                info!("Got transfer request");
                queue_tranfers(files, transfer, &tx, addr);
            }
            _ => {
                info!(%mess, "Invalid message");
            }
        }
    }
    Ok(())
}

fn send_file_list(sock: &UdpSocket, files: &RwLock<Arc<[FileEntry]>>, to_addr: SocketAddr) {
    let lock = files.read().unwrap();
    let packet = Proto::FileList(lock.clone());
    let buf = bincode::serialize(&packet).unwrap();
    sock.send_to(buf.as_ref(), to_addr).unwrap();
}

fn queue_tranfers(
    files: &RwLock<Arc<[FileEntry]>>,
    request: FileEntry,
    tx: &Sender<(FileEntry, SocketAddr)>,
    to_addr: SocketAddr,
) {
    let lock = files.read().unwrap();
    if !lock.contains(&request) {
        return;
    }
    tx.send((request, to_addr)).unwrap();
}

struct MessageStream<'a>(&'a UdpSocket);

impl<'a> Iterator for MessageStream<'a> {
    type Item = (Proto, SocketAddr);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 1500];
        loop {
            let (read, addr) = self.0.recv_from(&mut buffer).ok()?;
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
