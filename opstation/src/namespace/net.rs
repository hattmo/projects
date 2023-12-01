use cstr::cstr;
use libc::{__c_anonymous_ifr_ifru, c_int, exit, ifreq, ioctl, open, IFF_TUN, IFNAMSIZ, O_RDWR};

pub fn create_tun() -> c_int {
    unsafe {
        let fd = open(cstr!("/dev/net/tun").as_ptr(), O_RDWR);
        if fd < 0 {
            exit(-1)
        };
        let mut name = [0i8; IFNAMSIZ];
        "tun0"
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &b)| name[i] = b as i8);
        let ifreq = ifreq {
            ifr_name: name,
            ifr_ifru: __c_anonymous_ifr_ifru {
                ifru_flags: IFF_TUN as i16,
            },
        };
        ioctl(fd, 0x400454ca, &ifreq);
        fd
    }
}
