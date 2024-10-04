use std::os::fd::{FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStringExt;
use std::os::unix::net::AncillaryData::{ScmCredentials, ScmRights};
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

pub trait Fds {
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
