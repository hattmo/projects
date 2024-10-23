use cli::CliError;
use server::ServerError;

mod backend;
mod cli;
mod server;
mod util;

#[tokio::main]
async fn main() {
    match std::env::args().next().as_deref() {
        Some(command) if command.contains("wordlistctl") => {
            if let Err(err) = cli::main().await {
                match err {
                    CliError::Io(error) => println!("Cli failed: {error}"),
                }
            };
        }
        Some(_) => {
            if let Err(err) = server::main().await {
                match err {
                    ServerError::Io(error) => println!("Server failed: {error}"),
                    ServerError::Backend(_backend_error) => {
                        println!("Server failed: DB failure")
                    }
                }
            };
        }
        None => return,
    }
}
