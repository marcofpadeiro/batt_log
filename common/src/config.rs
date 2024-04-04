use std::time::Duration;

use home::home_dir;

const CONFIG_PATHS: [&str; 4] = [
    "~/.config/batt_log/config.toml",
    "~/.config/batt_log.toml",
    "~/.batt_log.toml",
    "/etc/batt_log/config.toml",
];

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub log_path: String,
    #[serde(with = "serde_humanize_rs")]
    pub polling_interval: Duration,
}

impl Config {
    fn default() -> Self {
        Self {
            log_path: "/var/log/batt_log.db".to_string(),
            polling_interval: Duration::from_secs(60),
        }
    }

    pub fn new() -> Self {
        CONFIG_PATHS
            .iter()
            .map(|path| replace_home(path))
            .find_map(|path| match std::fs::read_to_string(&path) {
                Ok(config_str) => match toml::from_str::<Config>(&config_str) {
                    Ok(config) => {
                        eprintln!("Using configuration: {}", path);
                        Some(config)
                    }
                    Err(_) => {
                        eprintln!("Invalid TOML format in config file: {}", path);
                        None
                    }
                },
                Err(_) => None,
            })
            .unwrap_or_else(|| {
                eprintln!("Failed to read or parse config file. Using default configuration.");
                Config::default()
            })
    }
}

fn replace_home(path: &str) -> String {
    if let Some(home) = home_dir() {
        path.replacen("~", home.to_str().unwrap(), 1)
    } else {
        path.to_string()
    }
}

mod serde_humanize_rs {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}
