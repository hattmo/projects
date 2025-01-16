#![feature(
    core_io_borrowed_buf,
    never_type,
    unwrap_infallible,
    maybe_uninit_as_bytes,
    read_buf
)]

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
    sync::RwLock,
};

use libc::{IN_CLOEXEC, IN_CREATE, IN_DELETE, IN_EXCL_UNLINK, IN_MOVE, IN_ONLYDIR};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
enum Proto {
    Request(Request),
    Response(Response),
}

#[derive(Serialize, Deserialize)]
enum Response {
    FileList(Vec<FileEntry>),
    FileChunk(FileChunk),
}

#[derive(Serialize, Deserialize)]
struct FileChunk {
    name: String,
    chunk: u64,
    content: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct FileEntry {
    hash: Vec<u8>,
    size: u128,
    name: String,
}

#[derive(Serialize, Deserialize)]
enum Request {
    Available,
    Transfer(Vec<String>),
}

fn main() -> IoResult<()> {
    let file_list = RwLock::new(Vec::new());
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 1234))?;
    sock.set_broadcast(true)?;
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::scope(|t| {
        [
            t.spawn(|| listen_job(&sock, &file_list)),
            t.spawn(|| watch_job(&file_list)),
        ]
        .into_iter()
        .for_each(|h| {
            h.join().unwrap().unwrap();
        });
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

fn watch_job(file_list: &RwLock<Vec<FileEntry>>) -> IoResult<()> {
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
            println!("Read error: {e:?}");
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
            .collect::<Vec<_>>();
        println!("{lock:?}");
    }
}

fn listen_job(sock: &UdpSocket, files: &RwLock<Vec<FileEntry>>) -> IoResult<()> {
    for (mess, addr) in MessageStream(&sock) {
        match mess {
            Proto::Request(Request::Available) => {
                send_file_list(&sock, files, addr);
            }
            Proto::Request(Request::Transfer(transfer)) => todo!(),
            Proto::Response(response) => todo!(),
        }
    }
    Ok(())
}

fn send_file_list(sock: &UdpSocket, files: &RwLock<Vec<FileEntry>>, to: SocketAddr) {
    let lock = files.read().unwrap();
    //TODO: Theres an extra clone here that i want to get rid of.
    let packet = Proto::Response(Response::FileList(lock.to_vec()));
    let buf = bincode::serialize(&packet).unwrap();
    sock.send_to(buf.as_ref(), to).unwrap();
}

fn queue_tranfers(files: &RwLock<Vec<FileEntry>>, requests: Vec<FileEntry>) -> IoResult<()> {
    for request in requests {
        let lock = files.read().unwrap();
        let Some(entry) = lock.iter().find(|entry| request == **entry) else {
            //TODO: Bad requests
            continue;
        };
    }
    Ok(())
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
                    println!("{err:?}");
                    continue;
                }
            };
            return Some((message, addr));
        }
    }
}
