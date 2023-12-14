use dirs::config_local_dir;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    // TODO: ignore certain relative paths
    pub ignore: Vec<String>,
}

#[allow(dead_code)]
pub fn config() -> anyhow::Result<Config> {
    let conf_dir = match config_local_dir() {
        Some(dir) => dir.join("rm_backup").join("config.toml"),
        None => anyhow::bail!("Unable to find local config directory."),
    };

    Ok(Figment::new()
        .merge(Toml::file(conf_dir))
        .merge(Env::prefixed("RM_BACKUP_"))
        .extract()?)
}
