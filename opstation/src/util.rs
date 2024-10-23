use std::io::{self, IoSlice, IoSliceMut};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStringExt;
use std::os::unix::net::AncillaryData::{ScmCredentials, ScmRights};
use std::os::unix::net::UnixDatagram;
use std::path::PathBuf;
use std::{ffi::CString, os::unix::net::SocketAncillary};

pub trait AsCString {
    fn as_cstring(&self) -> CString;
}

impl<T: ?Sized> AsCString for T
where
    T: ToOwned<Owned = PathBuf>,
{
    fn as_cstring(&self) -> CString {
        let path: PathBuf = self.to_owned();
        CString::new(path.into_os_string().into_vec()).expect("Unix paths are always c strings")
    }
}

trait Fds {
    fn fds(&self) -> Vec<OwnedFd>;
}

impl<'a> Fds for SocketAncillary<'a> {
    fn fds(&self) -> Vec<OwnedFd> {
        self.messages()
            .filter_map(Result::ok)
            .filter_map(|data| match data {
                ScmRights(rights) => Some(rights.collect::<Vec<_>>()),
                ScmCredentials(_) => None,
            })
            .flatten()
            .map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })
            .collect()
    }
}

pub trait SendAncillary {
    fn send_ancillary(&mut self, buf: &[u8], fds: &[impl AsRawFd]) -> io::Result<usize>;
}

impl SendAncillary for UnixDatagram {
    fn send_ancillary(&mut self, buf: &[u8], fds: &[impl AsRawFd]) -> io::Result<usize> {
        let bufs = [IoSlice::new(buf)];
        let mut ancillary_buf = [0; 255];
        let mut ancillary = SocketAncillary::new(&mut ancillary_buf);
        let fds: Vec<i32> = fds.iter().map(|i| i.as_raw_fd()).collect();
        if !ancillary.add_fds(&fds) {
            return Err(io::Error::other("Too much data for ancillary"));
        };
        self.send_vectored_with_ancillary(&bufs, &mut ancillary)
    }
}

pub trait ReceiveAncillary {
    fn recv_ancillary(&self, buf: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)>;
}

impl ReceiveAncillary for UnixDatagram {
    fn recv_ancillary(&self, buf: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        let mut bufs = [IoSliceMut::new(buf)];
        let mut ancillary_buf = [0; 255];
        let mut ancillary = SocketAncillary::new(&mut ancillary_buf);
        let (read, trunc) = self.recv_vectored_with_ancillary(&mut bufs, &mut ancillary)?;
        if trunc {
            return Err(io::Error::other("Control data was discarded"));
        }
        let fds = ancillary.fds();
        Ok((read, fds))
    }
}
