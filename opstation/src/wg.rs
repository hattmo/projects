use std::{
    ffi::CString,
    net::{IpAddr, ToSocketAddrs},
    ptr,
    time::Duration,
};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
#[allow(unnecessary_transmutes)]
mod raw {
    include!(concat!(env!("OUT_DIR"), "/wg_binding.rs"));
}
fn calloc<T>() -> *mut T {
    unsafe { libc::calloc(1, size_of::<T>()) as *mut T }
}

pub struct WGDevice {
    dev: *mut raw::wg_device,
}

pub struct Peer<'a> {
    peer: *mut raw::wg_peer,
    _parent: &'a mut WGDevice,
}

impl<'a> Peer<'a> {
    fn new(peer: *mut raw::wg_peer, parent: &'a mut WGDevice) -> Peer<'a> {
        Self {
            peer,
            _parent: parent,
        }
    }

    pub fn set_public_key(&mut self, key: [u8; 32]) {
        unsafe { (*self.peer).public_key = key }
        unsafe { (*self.peer).flags |= raw::wg_peer_flags_WGPEER_HAS_PUBLIC_KEY }
    }

    pub fn set_preshared_key(&mut self, key: [u8; 32]) {
        unsafe { (*self.peer).preshared_key = key }
        unsafe { (*self.peer).flags |= raw::wg_peer_flags_WGPEER_HAS_PRESHARED_KEY }
    }
    pub fn set_persistent_keepalive_interval(&mut self, interval: Duration) {
        let interval = interval.as_secs();
        let interval = if interval > u16::MAX as u64 {
            u16::MAX
        } else {
            interval as u16
        };
        unsafe { (*self.peer).persistent_keepalive_interval = interval }
        unsafe { (*self.peer).flags |= raw::wg_peer_flags_WGPEER_HAS_PERSISTENT_KEEPALIVE_INTERVAL }
    }

    pub fn add_allowed_ip(&mut self, network: IpAddr, cidr: u8) {
        unsafe {
            let allowed_ip: *mut raw::wg_allowedip = calloc();
            (*allowed_ip).cidr = cidr;
            match network {
                IpAddr::V4(addr) => {
                    (*allowed_ip).family = libc::AF_INET as u16;
                    (*allowed_ip).__bindgen_anon_1 = raw::wg_allowedip__bindgen_ty_1 {
                        ip4: raw::in_addr {
                            s_addr: addr.to_bits().to_be(),
                        },
                    };
                }
                IpAddr::V6(addr) => {
                    (*allowed_ip).family = libc::AF_INET6 as u16;
                    (*allowed_ip).__bindgen_anon_1 = raw::wg_allowedip__bindgen_ty_1 {
                        ip6: raw::in6_addr {
                            __in6_u: raw::in6_addr__bindgen_ty_1 {
                                __u6_addr8: addr.octets(),
                            },
                        },
                    };
                }
            }
            let last_allowedip = (*self.peer).last_allowedip;
            if last_allowedip.is_null() {
                (*self.peer).first_allowedip = allowed_ip;
            } else {
                (*(*self.peer).last_allowedip).next_allowedip = allowed_ip;
            }
            (*self.peer).last_allowedip = allowed_ip;
        }
    }

    pub fn set_endpoint(&mut self, addr: impl ToSocketAddrs) {
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        let endpoint = match addr {
            std::net::SocketAddr::V4(addr) => raw::wg_endpoint {
                addr4: raw::sockaddr_in {
                    sin_family: libc::AF_INET as u16,
                    sin_port: addr.port().to_be(),
                    sin_addr: raw::in_addr {
                        s_addr: addr.ip().to_bits().to_be(),
                    },
                    sin_zero: Default::default(),
                },
            },
            std::net::SocketAddr::V6(addr) => {
                let mut ip = addr.ip().octets();
                ip.reverse();
                let port = addr.port().to_be();
                raw::wg_endpoint {
                    addr6: raw::sockaddr_in6 {
                        sin6_family: libc::AF_INET6 as u16,
                        sin6_port: port,
                        sin6_addr: raw::in6_addr {
                            __in6_u: raw::in6_addr__bindgen_ty_1 { __u6_addr8: ip },
                        },
                        sin6_flowinfo: 0,
                        sin6_scope_id: 0,
                    },
                }
            }
        };
        unsafe { (*self.peer).endpoint = endpoint };
    }
}

impl WGDevice {
    pub fn new(name: &str) -> Self {
        let name: CString = name.parse().unwrap();

        unsafe {
            raw::wg_add_device(name.as_ptr());
            let mut dev: *mut raw::wg_device = ptr::null_mut();
            raw::wg_get_device(ptr::from_mut(&mut dev), name.as_ptr());
            Self { dev }
        }
    }

    pub fn new_peer(&mut self) -> Peer {
        let peer: *mut raw::wg_peer = calloc();
        unsafe {
            let last_peer = (*self.dev).last_peer;
            if last_peer.is_null() {
                (*self.dev).first_peer = peer;
            } else {
                (*(*self.dev).last_peer).next_peer = peer;
            }
            (*self.dev).last_peer = peer;
        }
        Peer::new(peer, self)
    }
    pub fn set_private_key(&mut self, key: [u8; 32]) {
        unsafe { (*self.dev).private_key = key };
        unsafe { (*self.dev).flags |= raw::wg_device_flags_WGDEVICE_HAS_PRIVATE_KEY };
    }
    pub fn set_public_key(&mut self, key: [u8; 32]) {
        unsafe { (*self.dev).public_key = key };
        unsafe { (*self.dev).flags |= raw::wg_device_flags_WGDEVICE_HAS_PUBLIC_KEY };
    }
    pub fn set_listen_port(&mut self, port: u16) {
        unsafe { (*self.dev).listen_port = port };
        unsafe { (*self.dev).flags |= raw::wg_device_flags_WGDEVICE_HAS_LISTEN_PORT };
    }
    pub fn set_fw_mark(&mut self, fwmark: u32) {
        unsafe { (*self.dev).fwmark = fwmark };
        unsafe { (*self.dev).flags |= raw::wg_device_flags_WGDEVICE_HAS_FWMARK };
    }
    pub fn commit(&mut self) {
        unsafe { raw::wg_set_device(self.dev) };
    }
}

impl Drop for WGDevice {
    fn drop(&mut self) {
        unsafe {
            raw::wg_del_device((*self.dev).name.as_ptr());
            raw::wg_free_device(self.dev);
        }
    }
}

pub fn generate_key_pair() -> ([u8; 32], [u8; 32]) {
    let mut private_key = [0u8; 32];
    let mut public_key = [0u8; 32];
    unsafe { raw::wg_generate_private_key(private_key.as_mut_ptr()) };
    unsafe { raw::wg_generate_public_key(public_key.as_mut_ptr(), private_key.as_ptr()) };
    return (public_key, private_key);
}
