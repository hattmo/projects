use std::{
    io::{Result, Write},
    net::TcpListener,
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    fastcgi::run_tcp(
        |mut req| {
            req.params().for_each(|(k, v)| println!("{k} = {v}"));
            let res = std::io::read_to_string(req.stdin()).unwrap();
            println!("__STDIN__\n{res}\n__END_STDIN__");
            let mut out = req.stdout();
            writeln!(out, "Content-type: text/html\r").unwrap();
            writeln!(out, "\r").unwrap();

            write!(out, "Hello world").unwrap();
            let mut err = req.stderr();
            write!(err, "ERROR MESSAGE FROM HATTMO").unwrap();
            req.exit(0);
        },
        &listener,
    );
    Ok(())
}
