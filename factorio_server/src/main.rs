#![feature(lazy_cell)]
use anyhow::Result;
use clap::Parser;
use std::sync::LazyLock;

use service::Settings;

mod service;
mod settings;

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::parse());

#[derive(Parser)]
struct Config {
    #[clap(short, long, default_value = "./factorio")]
    factorio_path: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let mut service = service::Service::new();
    let map_gen_settings = serde_json::from_str(include_str!("map_gen_settings.json"))?;
    let map_settings = serde_json::from_str(include_str!("map_gen_settings.json"))?;
    let settings = Settings::new(&map_gen_settings, &map_settings).await?;
    let save = settings.create_save().await?;
    service.start_server(&save).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use serde_json;
    #[derive(Serialize, Deserialize, Debug)]
    struct Foo {
        #[serde(skip_serializing_if = "Option::is_none")]
        bar: Option<u32>,
    }

    #[test]
    fn test() {
        let mut foo: Foo = serde_json::from_str("{}").unwrap();
        println!("{:?}", foo);
        foo.bar = Some(1);
        let out = serde_json::to_string_pretty(&foo).unwrap();
        println!("{}", out);
    }
}
