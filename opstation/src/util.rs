use std::os::unix::ffi::OsStringExt;
use std::{
    ffi::{CString, NulError},
    path::Path,
};

pub trait AsCString {
    fn as_cstring(&self) -> Result<CString, NulError>;
}

impl<T> AsCString for T
where
    T: AsRef<Path>,
{
    fn as_cstring(&self) -> Result<CString, NulError> {
        let path: &Path = self.as_ref();
        let path = path.to_owned();
        CString::new(path.into_os_string().into_vec())
    }
}
