use rustls::RootCertStore;
use std::io::{prelude::*, BufReader};
fn foo() {
    let cert = &[0u8, 2, 1][..];
    let certs = &[cert][..];
    let mut store = RootCertStore::empty();
    store.add_parsable_certificates(certs);
    let conf = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(store)
        .with_no_client_auth();
    let mut sess =
        rustls::ClientConnection::new(conf.into(), "localhost".try_into().unwrap()).unwrap();
    let vec = vec![0u8; 64];
    let mut buf = vec.as_slice();
    sess.read_tls(&mut buf);
}

#[cfg(test)]
mod test {
    use std::io::Read;

    #[test]
    fn test() {
        let in_buf = vec![1u8; 64];
        let mut in_slice = &in_buf[..20];

        let mut out_buf = [0u8; 10];
        let out_slice = out_buf.as_mut_slice();

        let _ = in_slice.read(out_slice).unwrap(); 
    }
}
