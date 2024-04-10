use core::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

#[allow(non_camel_case_types)]
#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: [u8; 4],
}

struct Socket;

// pub fn connect(addr: SocketAddr) -> Socket {
//     match addr {
//         SocketAddr::V4(addr) => {
//             let addr_in = sockaddr_in {
//                 sin_family: 2,
//                 sin_port: addr.port().to_be(),
//                 sin_addr: addr.ip().octets(),
//             };
//             let addr_ptr = &addr_in as *const sockaddr_in;
//         }
//         SocketAddr::V6(addr) => {
//             // ...
//         }
//     }
// }

impl Future for Socket {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
