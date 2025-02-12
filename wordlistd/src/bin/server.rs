use std::error::Error;
use tracing::info;
use wordlistd::{
    backend,
    frontend::{self, server},
    BackendConfig, FrontendConfig, ServerConfig,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    info!("Starting server");
    let conf = ServerConfig {
        be: BackendConfig::Sqllite {
            path: "test_db".into(),
        },
        fe: FrontendConfig::Dbus,
    };
    info!(?conf, "Using configuration");
    let mut fe = frontend::get_server_frontend().await?;
    let mut be = backend::get_backend(&conf).await?;
    info!("Server started");
    while let Some(req) = fe.get_request().await {
        match req {
            server::Request::AddWord(add_word) => {
                be.add_word(&add_word.word, &add_word.tags).await?;
            }
            server::Request::GetWords(get_words) => {
                let words = be.get_words(&get_words.tags).await?;
                get_words.reply(words);
            }
        }
    }
    Ok(())
}
