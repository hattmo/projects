use crate::settings::{MapGenSettings, MapSettings};
use crate::CONFIG;
use anyhow::{bail, Ok, Result};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};

pub struct Service {
    child: Option<Child>,
}

impl Service {
    pub fn new() -> Self {
        Service { child: None }
    }

    pub async fn start_server(&mut self, save: &Save) -> Result<()> {
        let mut child = Command::new(format!("{}/bin/x64/factorio", CONFIG.factorio_path))
            .arg("--start-server")
            .arg(save.save_path.as_os_str())
            .kill_on_drop(true)
            .spawn()?;
        child.wait().await?;
        self.child = Some(child);
        Ok(())
    }
    pub async fn stop_server(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill().await?;
        }
        Ok(())
    }
}

enum Root {
    Settings,
    Saves,
}

pub struct Settings {
    map_gen_path: PathBuf,
    map_path: PathBuf,
}

impl Settings {
    pub async fn new(
        map_gen_settings: &MapGenSettings,
        map_settings: &MapSettings,
    ) -> Result<Self> {
        let setting_root = get_new_root(Root::Settings).await?;
        let map_gen_path = setting_root.join("map_gen_settings.json");
        let mut map_gen_file = File::create(&map_gen_path).await?;
        let buf = serde_json::to_vec(map_gen_settings)?;
        map_gen_file.write_all(&buf).await?;

        let map_path = setting_root.join("map_settings.json");
        let mut map_file = File::create(&map_path).await?;
        let buf = serde_json::to_vec(map_settings)?;
        map_file.write_all(&buf).await?;

        Ok(Self {
            map_gen_path,
            map_path,
        })
    }

    pub async fn create_save(&self) -> Result<Save> {
        tracing::info!("Creating new save");
        let save_root = get_new_root(Root::Saves).await?;
        let save_path = save_root.join("save.zip");
        let command_res = Command::new(format!("{}/bin/x64/factorio", CONFIG.factorio_path))
            .arg("--create")
            .arg(save_path.as_os_str())
            .arg("--map-gen-settings")
            .arg(self.map_gen_path.as_os_str())
            .arg("--map-settings")
            .arg(self.map_path.as_os_str())
            .output()
            .await?;
        println!(
            "----STDOUT---\n{}
            \n----STDERR---\n{}",
            String::from_utf8_lossy(&command_res.stdout),
            String::from_utf8_lossy(&command_res.stderr)
        );
        if command_res.status.success() {
            Ok(Save { save_path })
        } else {
            bail!("Failed to create new save")
        }
    }
}

pub struct Save {
    save_path: PathBuf,
}

async fn get_new_root(root: Root) -> Result<PathBuf> {
    let root_type = match root {
        Root::Settings => "settings",
        Root::Saves => "saves",
    };
    let settings_root = PathBuf::new().join(format!(
        "{}/{}/{}/",
        CONFIG.factorio_path,
        root_type,
        gen_id()
    ));
    create_dir_all(&settings_root).await?;
    Ok(settings_root)
}

fn gen_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}
