use std::{io, net::SocketAddr, str::FromStr};

use trust_dns_client::{
    client::{Client, SyncClient},
    rr::{DNSClass, Name, RecordType},
    udp::UdpClientConnection,
};

fn main() -> Result<(), io::Error> {
    let conn = UdpClientConnection::new(SocketAddr::from_str("51.222.39.63:53").unwrap()).unwrap();
    let client = SyncClient::new(conn);
    let res = client
        .query(
            &Name::from_str("megacorpone.com").unwrap(),
            DNSClass::IN,
            RecordType::AXFR,
        )
        .unwrap();
    println!("{:?}", res);
    // let mut res_conf = ResolverConfig::new();
    // res_conf.add_name_server(NameServerConfig::new(
    //     ,
    //     Protocol::Udp,
    // ));
    // res_conf.
    // let mut res_opt = ResolverOpts::default();
    // res_opt.recursion_desired = false;
    // let res = Resolver::new(res_conf, res_opt)?;
    // let response = res.lookup("megacorpone.com", RecordType::AXFR)?;
    // for item in response.into_iter() {
    //     println!("{}", item);
    // }
    Ok(())
}
