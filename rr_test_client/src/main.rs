#![feature(core_io_borrowed_buf)]
use clap::Parser as ClapParser;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::streaming::{tag, take},
    combinator::{all_consuming, consumed, map, map_parser, map_res, rest, value},
    error::Error,
    multi::length_value,
    number::streaming::be_u32,
    sequence::preceded,
};
use std::{
    collections::HashMap,
    io::{self, BorrowedBuf, Read, Write},
    net::TcpStream,
    num::ParseIntError,
    path::{Path, PathBuf},
    str::Utf8Error,
    thread,
    time::Duration,
};

#[derive(ClapParser)]
struct Args {
    host: String,
    port: u16,
}

#[derive(Clone, Copy, Debug)]
enum Command<'a> {
    Sleep(Duration),
    Upload(Upload<'a>),
    Download(Download<'a>),
    Hostname,
    Netstat,
    Proclist,
    Invoke(&'a [u8]),
    Shutdown,
}

#[derive(Clone, Copy, Debug)]
struct Upload<'a> {
    dst: &'a Path,
    content: &'a [u8],
}

#[derive(Clone, Copy, Debug)]
struct Download<'a> {
    src: &'a Path,
}

enum CommandError {
    InvalidUtf8,
    ParseIntError,
}
impl From<Utf8Error> for CommandError {
    fn from(_: Utf8Error) -> Self {
        CommandError::InvalidUtf8
    }
}
impl From<ParseIntError> for CommandError {
    fn from(_: ParseIntError) -> Self {
        CommandError::ParseIntError
    }
}

impl<'a> Command<'a> {
    pub fn from_bytes(buf: &'a [u8]) -> IResult<&'a [u8], (&'a [u8], Self)> {
        consumed(packet_bytes(alt((
            Self::sleep,
            Self::upload,
            Self::download,
            Self::invoke,
            value(
                Command::Hostname,
                (
                    len_bytes(tag(b"hostname\0".as_slice())),
                    len_bytes(tag(b"\0".as_slice())),
                ),
            ),
            value(
                Command::Netstat,
                (
                    len_bytes(tag(b"netstat\0".as_slice())),
                    len_bytes(tag(b"\0".as_slice())),
                ),
            ),
            value(
                Command::Proclist,
                (
                    len_bytes(tag(b"proclist\0".as_slice())),
                    len_bytes(tag(b"\0".as_slice())),
                ),
            ),
            value(
                Command::Shutdown,
                (
                    len_bytes(tag(b"shutdown\0".as_slice())),
                    len_bytes(tag(b"\0".as_slice())),
                ),
            ),
        ))))
        .parse(buf)
    }

    fn sleep(buf: &'a [u8]) -> IResult<&'a [u8], Self> {
        preceded(
            len_bytes(tag(b"sleep\0".as_slice())),
            len_bytes(map_res(rest, |arg| -> Result<Self, CommandError> {
                Ok(Self::Sleep(Duration::from_secs(
                    str::from_utf8(arg)?.parse()?,
                )))
            })),
        )
        .parse(buf)
    }

    fn upload(buf: &'a [u8]) -> IResult<&'a [u8], Self> {
        preceded(
            len_bytes(tag(b"upload\0".as_slice())),
            map(
                len_bytes((
                    map_res(len_bytes(rest), |path| -> Result<&Path, CommandError> {
                        Ok(Path::new(str::from_utf8(path)?))
                    }),
                    len_bytes(rest),
                )),
                |(dst, content)| Self::Upload(Upload { dst, content }),
            ),
        )
        .parse(buf)
    }

    fn download(buf: &'a [u8]) -> IResult<&'a [u8], Self> {
        preceded(
            len_bytes(tag(b"download\0".as_slice())),
            map(
                len_bytes(map_res(
                    len_bytes(rest),
                    |path| -> Result<&Path, CommandError> { Ok(Path::new(str::from_utf8(path)?)) },
                )),
                |src| Self::Download(Download { src }),
            ),
        )
        .parse(buf)
    }

    fn invoke(buf: &'a [u8]) -> IResult<&'a [u8], Self> {
        preceded(
            len_bytes(tag(b"invoke\0".as_slice())),
            map(len_bytes(rest), |cmd| Self::Invoke(cmd)),
        )
        .parse(buf)
    }
}
fn packet_bytes<'a, P, O>(parser: P) -> impl Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>
where
    P: Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>,
{
    map_parser(be_u32.flat_map(|i| take(i - 4)), all_consuming(parser))
}

fn len_bytes<'a, P, O>(parser: P) -> impl Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>
where
    P: Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>,
{
    length_value(be_u32, all_consuming(parser))
}

fn main() {
    let mut fs = HashMap::new();
    let Args { host, port } = Args::parse();

    'outer: loop {
        let mut in_buf = [0u8; 20_000];
        let mut out_buf = [0u8; 20_000];
        let mut filled = 0;
        let Ok(mut sock) = TcpStream::connect((host.as_str(), port)) else {
            println!("Failed to connect, sleeping for 5 secs");
            thread::sleep(Duration::from_secs(5));
            continue;
        };
        println!("Connected to server");
        let Ok(checkin) = write_response(0, b"roadrunner checkin\0", &mut out_buf[..]) else {
            thread::sleep(Duration::from_secs(5));
            continue;
        };
        sock.write_all(checkin).unwrap();
        println!("Sent checkin");
        loop {
            let Ok(read) = sock.read(&mut in_buf[filled..]) else {
                println!("Failed to read from server reconnecting");
                thread::sleep(Duration::from_secs(5));
                continue 'outer;
            };
            println!("Read {read} bytes from server");
            filled += read;
            let (extra, (consumed, cmd)) = match Command::from_bytes(&in_buf[..filled]) {
                Ok(ok) => ok,
                Err(err) => {
                    println!("{err}");
                    if !err.is_incomplete() {
                        println!("Error parsing command reconnecting");
                        thread::sleep(Duration::from_secs(5));
                        continue 'outer;
                    }
                    println!("Not enough bytes from server reading again");
                    continue;
                }
            };
            println!("Got command from server: {cmd:?}");
            let out = handle_command(&mut fs, cmd, &mut out_buf[..]).unwrap();
            sock.write_all(out).unwrap();
            if let Command::Shutdown = cmd {
                println!("Exiting");
                return;
            }
            let consumed_len = consumed.len();
            let extra_len = extra.len();
            in_buf.copy_within(..extra_len, consumed_len);
            filled = extra_len;
        }
    }
}

fn handle_command<'a, 'b>(
    fs: &mut HashMap<PathBuf, Vec<u8>>,
    cmd: Command<'a>,
    out_buf: &'b mut [u8],
) -> io::Result<&'b [u8]> {
    match cmd {
        Command::Sleep(duration) => {
            thread::sleep(duration);
            write_response(0, b"Sleep Successful", out_buf)
        }
        Command::Upload(Upload { dst, content }) => {
            fs.insert(dst.to_owned(), content.to_owned());
            write_response(0, b"Upload Successful", out_buf)
        }
        Command::Download(Download { src }) => {
            let content = fs.entry(src.to_owned()).or_insert(b"Mock File".to_vec());
            write_response(0, &content, out_buf)
        }
        Command::Hostname => write_response(0, b"Totally Hostname Response", out_buf),
        Command::Netstat => write_response(0, b"Totally Netstat Response", out_buf),
        Command::Proclist => write_response(0, b"Totally Proclist Response", out_buf),
        Command::Invoke(_) => write_response(0, b"Totally Invoke Response", out_buf),
        Command::Shutdown => write_response(0, b"shutting down", out_buf),
    }
}

fn write_response<'a, 'b>(
    ret_code: u32,
    message: &'b [u8],
    buf: &'a mut [u8],
) -> io::Result<&'a [u8]> {
    let mut buf: BorrowedBuf = buf.into();
    let mut cursor = buf.unfilled();
    let total_len = 4 + 4 + 4 + message.len();
    cursor.write_all(&mut (total_len as u32).to_be_bytes())?;
    cursor.write_all(&mut ret_code.to_be_bytes())?;
    cursor.write_all(&mut (message.len() as u32).to_be_bytes())?;
    cursor.write_all(message)?;
    Ok(buf.into_filled())
}
