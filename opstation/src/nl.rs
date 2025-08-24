use std::{
    ffi::CStr,
    fmt::{self, Display, Formatter},
    ptr,
};

use libc::{__errno_location, AF_INET, AF_UNSPEC, NETLINK_ROUTE, NLM_F_CREATE};

use raw::nl_sock;

use crate::nl::raw::{nl_geterror, nl_socket_alloc, nl_socket_free, nl_syserr2nlerr};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
#[allow(unnecessary_transmutes)]
mod raw {
    include!(concat!(env!("OUT_DIR"), "/nl_binding.rs"));
}

struct NlError {
    errno: i32,
}

impl From<i32> for NlError {
    fn from(value: i32) -> Self {
        Self { errno: value }
    }
}
impl Display for NlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let err_str = unsafe { nl_geterror(self.errno) };
        let out_str = unsafe { CStr::from_ptr(err_str) }
            .to_str()
            .map_err(|_| fmt::Error)?;
        write!(f, "{out_str}")
    }
}

struct NlContext {
    sock: *mut nl_sock,
}

impl Drop for NlContext {
    fn drop(&mut self) {
        unsafe { nl_socket_free(self.sock) };
    }
}

impl NlContext {
    pub fn new() -> Result<Self, NlError> {
        let sock = unsafe { nl_socket_alloc() };
        if sock.is_null() {
            let errno = unsafe { nl_syserr2nlerr(*__errno_location()) };
            return Err(NlError { errno });
        }
        Ok(NlContext { sock })
    }
    pub fn create_vxlan(&self) {}
}

pub unsafe fn make_vx_lan() {
    let sock = raw::nl_socket_alloc();
    raw::nl_connect(sock, NETLINK_ROUTE);

    let mut cache = ptr::null_mut();
    raw::rtnl_link_alloc_cache(sock, AF_UNSPEC, ptr::from_mut(&mut cache));
    let link = raw::rtnl_link_vxlan_alloc();

    let mut local = ptr::null_mut();
    raw::nl_addr_parse(
        c"172.168.212.122".as_ptr(),
        AF_INET,
        ptr::from_mut(&mut local),
    );
    raw::rtnl_link_vxlan_set_local(link, local);

    let mut remote = ptr::null_mut();
    raw::nl_addr_parse(
        c"172.168.212.125".as_ptr(),
        AF_INET,
        ptr::from_mut(&mut remote),
    );
    raw::rtnl_link_vxlan_set_group(link, remote);

    raw::rtnl_link_vxlan_set_id(link, 1234);

    let ifindex = raw::rtnl_link_name2i(cache, c"eth0".as_ptr());
    raw::rtnl_link_set_link(link, ifindex);
    raw::rtnl_link_set_name(link, c"vxlan0".as_ptr());
    raw::rtnl_link_add(sock, link, NLM_F_CREATE);

    raw::nl_cache_free(cache);
    raw::nl_addr_put(local);
    raw::nl_addr_put(remote);
    raw::rtnl_link_put(link);
    raw::nl_socket_free(sock);
}
