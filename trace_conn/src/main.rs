use std::{collections::HashMap, mem, net::Ipv4Addr, sync::Arc};

use axum::{extract::State, response::Html, routing::get, Router};

use clap::Parser;
use tokio::{main, spawn, sync::Mutex, task::spawn_blocking};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    interface: Option<i32>,
    #[arg(short, long)]
    listen: Ipv4Addr,
    #[arg(short, long)]
    port: u16,
}

/* enum Geo {
    Resolved(String),
    Unresolved,
    Private,
}
*/

struct ConnectionData {
    instances: u64,
    src_name: String,
    //   src_geo: Geo,
    dst_name: String,
    //   dst_geo: Geo,
}

type CountMap = Arc<Mutex<HashMap<[Ipv4Addr; 2], ConnectionData>>>;

#[main]
async fn main() {
    let args = Args::parse();
    let counts: CountMap = Arc::new(Mutex::new(HashMap::new()));
    let counts_capture = counts.clone();
    let capture_handle = spawn_blocking(move || capture_job(args.interface, counts_capture));
    let web_handle = spawn(web_job(args.listen, args.port, counts));
    let _ = capture_handle.await;
    let _ = web_handle.await;
}

async fn web_job(ip: Ipv4Addr, port: u16, counts: CountMap) {
    println!("starting server on {ip}:{port}");
    let app = Router::new()
        .route("/", get(root))
        .route("/index.html", get(root))
        .route("/data.html", get(data))
        .route("/index.css", get(css))
        .with_state(counts);
    let listener = tokio::net::TcpListener::bind((ip, port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    include_str!("index.html").into()
}

async fn css() -> Html<&'static str> {
    include_str!("index.css").into()
}

async fn data(state: State<CountMap>) -> Html<String> {
    let counts = state.lock().await;
    let mut count_vec: Vec<_> = counts.iter().collect();
    count_vec.sort_by(
        |([from_a, to_a], _), ([from_b, to_b], _)| match from_a.cmp(from_b) {
            Ordering::Equal => to_a.cmp(to_b),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        },
    );
    count_vec
        .into_iter()
        .map(
            |(
                [_, _],
                ConnectionData {
                    instances,
                    src_name,
                    dst_name,
                },
            )| {
                format!("<tr><td>{src_name}</td><td>{dst_name}</td><td>{instances}</td></tr>")
            },
        )
        .collect::<String>()
        .into()
}

fn capture_job(inf: Option<i32>, counts: CountMap) {
    let mut packet_buf = PacketBuf::new();
    let sock = match RawSocket::new(inf) {
        Ok(sock) => sock,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    loop {
        sock.get_packet(&mut packet_buf).unwrap();
        let packet = packet_buf.parse().unwrap();
        match packet.payload {
            Payload::IPv4(ip) => {
                let src = u32::from_be_bytes(ip.src_addr.try_into().unwrap());
                let dst = u32::from_be_bytes(ip.dst_addr.try_into().unwrap());
                let src = Ipv4Addr::from_bits(src);
                let dst = Ipv4Addr::from_bits(dst);
                let mut key = [src, dst];
                key.sort_by(sort_ipv4);
                let mut lock = counts.blocking_lock();
                let item = lock
                    .entry(key)
                    .or_insert_with_key(|[src, dst]| ConnectionData {
                        instances: 0,
                        src_name: get_domain_name(src),
                        dst_name: get_domain_name(dst),
                    });
                item.instances += 1;
            }
            Payload::Unknown => {}
        }
    }
}

use std::{
    cmp::Ordering, error::Error, ffi::CStr, fmt::Display, mem::transmute, net::UdpSocket,
    os::fd::FromRawFd, usize,
};

use libc::{
    bind, c_char, getnameinfo, sockaddr, sockaddr_in, sockaddr_ll, AF_INET, AF_PACKET, ETH_P_ALL,
    NI_MAXHOST, SOCK_RAW,
};

const ERR_BUF_LEN: libc::size_t = 255;

#[derive(Debug)]
pub struct ParseFail;

impl<T> From<T> for ParseFail
where
    T: Error,
{
    fn from(_value: T) -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct PacketBuf {
    len: usize,
    buf: [u8; 1500],
}

impl Display for PacketBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = &self.buf[0..self.len];
        let data_str: String = data
            .into_iter()
            .map(|b| {
                if b.is_ascii_alphanumeric() {
                    String::from(*b as char)
                } else {
                    format!("{{0x{b:02.2x}}}")
                }
            })
            .collect();
        write!(f, "{data_str}")
    }
}

impl PacketBuf {
    pub fn new() -> Self {
        Self {
            len: 0,
            buf: [0; 1500],
        }
    }

    pub fn parse(&self) -> Result<Packet, ParseFail> {
        let buf = &self.buf;
        if buf.len() <= 14 {
            return Err(ParseFail);
        }
        //let dst_mac = &buf[0..6];
        //let src_mac = &buf[6..12];
        let ether_type = u16::from_be_bytes((&buf[12..14]).try_into().or(Err(ParseFail))?);
        let payload = match ether_type {
            0x0800 => Payload::IPv4(IPv4::parse(&buf[14..])?),
            _ => Payload::Unknown,
        };
        Ok(Packet {
            //dst_mac,
            //src_mac,
            payload,
        })
    }
}

#[derive(Debug)]
pub struct Packet<'a> {
    //pub dst_mac: &'a [u8],
    //pub src_mac: &'a [u8],
    pub payload: Payload<'a>,
}

#[derive(Debug)]
pub enum Payload<'a> {
    IPv4(IPv4<'a>),
    Unknown,
}

#[derive(Debug)]
pub struct IPv4<'a> {
    pub src_addr: &'a [u8],
    pub dst_addr: &'a [u8],
}

impl<'a> IPv4<'a> {
    fn parse(buf: &'a [u8]) -> Result<Self, ParseFail> {
        Ok(Self {
            src_addr: &buf[12..16],
            dst_addr: &buf[16..20],
        })
    }
}

#[derive(Debug)]
pub struct OsError {
    buf: [libc::c_char; ERR_BUF_LEN],
}

impl OsError {
    fn get() -> Self {
        let mut out = OsError {
            buf: [0; ERR_BUF_LEN],
        };
        let errno = unsafe { *libc::__errno_location() };
        unsafe { libc::strerror_r(errno, out.buf.as_mut_ptr(), ERR_BUF_LEN - 1) };
        out
    }
}

impl Display for OsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mess = unsafe { CStr::from_ptr(self.buf.as_ptr()) }
            .to_str()
            .expect("is always valid");
        write!(f, "Os error: {mess}")
    }
}

impl Error for OsError {}

pub struct RawSocket {
    sock: UdpSocket,
}

impl RawSocket {
    pub fn new(interface: Option<i32>) -> Result<Self, OsError> {
        reset_errno();
        let ret = unsafe { libc::socket(AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be()) };
        if ret < 0 {
            return Err(OsError::get());
        }

        let sock = unsafe { UdpSocket::from_raw_fd(ret) };

        let mut addr: sockaddr_ll = unsafe { std::mem::zeroed() };
        let addr_ptr: *const sockaddr_ll = &addr;
        let addr_ptr: *const sockaddr = unsafe { transmute(addr_ptr) };
        addr.sll_family = AF_PACKET as u16;
        addr.sll_protocol = (ETH_P_ALL as u16).to_be();
        addr.sll_ifindex = interface.unwrap_or(0);

        reset_errno();
        let ret = unsafe { bind(ret, addr_ptr, size_of::<sockaddr_ll>() as u32) };
        if ret < 0 {
            return Err(OsError::get());
        }
        Ok(RawSocket { sock })
    }

    pub fn get_packet(&self, packet: &mut PacketBuf) -> std::io::Result<()> {
        let len = self.sock.recv(&mut packet.buf)?;
        packet.len = len;
        Ok(())
    }
}

fn reset_errno() {
    unsafe {
        let errno_ptr = libc::__errno_location();
        *errno_ptr = 0;
    }
}

fn get_domain_name(ip: &Ipv4Addr) -> String {
    let mut addr: sockaddr_in = unsafe { mem::zeroed() };
    addr.sin_family = AF_INET as u16;
    addr.sin_addr.s_addr = ip.to_bits().to_be();
    let addr_ptr: *const sockaddr_in = &addr;
    let addr_ptr: *const sockaddr = unsafe { transmute(addr_ptr) };
    let mut host_buf = [0i8; NI_MAXHOST as usize];
    let host_buf_ptr = (&mut host_buf) as *mut c_char;
    reset_errno();
    let ret = unsafe {
        getnameinfo(
            addr_ptr,
            size_of::<sockaddr_in>() as u32,
            host_buf_ptr,
            NI_MAXHOST,
            std::ptr::null_mut(),
            0,
            0,
        )
    };
    if ret != 0 {
        let err = OsError::get();
        println!("{err}");
        return ip.to_string();
    }
    unsafe { CStr::from_ptr(host_buf_ptr) }
        .to_string_lossy()
        .to_string()
}

fn sort_ipv4(a: &Ipv4Addr, b: &Ipv4Addr) -> Ordering {
    match (a.is_private(), b.is_private()) {
        (true, true) | (false, false) => a.cmp(b),
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
    }
}
