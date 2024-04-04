use std::time::Duration;

use home::home_dir;

const CONFIG_PATHS: [&str; 2] = [
    "~/.config/batt_log/config.toml",
    "/etc/batt_log/config.toml",
];

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub db_path: String,
    #[serde(with = "serde_humanize_rs")]
    pub polling_interval: Duration,
}

impl Config {
    fn default() -> Self {
        Self {
            db_path: "/var/lib/batt_log/log.db".to_string(),
            polling_interval: Duration::from_secs(60),
        }
    }

    pub fn new() -> Self {
        let config = CONFIG_PATHS
            .iter()
            .find(|path| std::path::Path::new(&replace_home(path)).exists())
            .unwrap_or(&CONFIG_PATHS[1]);

        let config = std::fs::read_to_string(config).unwrap_or_else(|_| {
            eprintln!("Failed to read config file. Using default configuration.");
            toml::to_string(&Config::default()).unwrap()
        });

        toml::from_str(&config).unwrap_or_else(|_| {
            eprintln!("Failed to parse config file. Using default configuration.");
            Config::default()
        })
    }
}

fn replace_home(path: &str) -> String {
    if let Some(home) = home_dir() {
        path.replace("~", home.to_str().unwrap())
    } else {
        path.to_string()
    }
}

mod serde_humanize_rs {
    use serde::{self, Deserialize, Deserializer, Serializer};
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
