#![allow(clippy::module_name_repetitions)]
use std::{
    error::Error,
    path::{Path, PathBuf},
    time::Duration,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub hosts: Vec<ConfigItem>,
    pub api_key: String,
    pub ip_address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ConfigItem {
    pub name: String,
    pub responses: Vec<SeverityItem>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SeverityItem {
    pub severity: Severity,
    pub response: AlarmResponseTypes,
    pub repeating: Option<Duration>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Default, Debug, Clone)]
pub enum Severity {
    #[default]
    Low,
    Middle,
    High,
    Test,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Middle => write!(f, "Medium"),
            Self::High => write!(f, "High"),
            Self::Test => write!(f, "Test"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct FileArguments {
    pub file_path: PathBuf,
    pub run_directory: Option<PathBuf>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlarmResponseTypes {
    Sound(FileArguments),
    Log(String),
    File(FileArguments),
}

impl Default for AlarmResponseTypes {
    fn default() -> Self {
        Self::Log("Defaut log".to_string())
    }
}

pub fn prarse(path: PathBuf) -> Result<Config, Box<dyn Error>> {
    Ok::<Config, Box<dyn Error>>(serde_yaml::from_str(std::str::from_utf8(&std::fs::read(
        path,
    )?)?)?)
}
pub fn generate_default(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        ip_address: "127.0.0.1".to_string(),
        port: 5000,
        api_key: "123".to_string(),
        hosts: vec![ConfigItem {
            name: "Host".to_string(),
            responses: vec![
                SeverityItem {
                    severity: Severity::Low,
                    response: AlarmResponseTypes::Log("Log error".to_string()),
                    repeating: None,
                },
                SeverityItem {
                    severity: Severity::Middle,
                    response: AlarmResponseTypes::Sound(FileArguments {
                        file_path: "~/Music/test.mp3".into(),
                        run_directory: None,
                    }),

                    repeating: None,
                },
                SeverityItem {
                    severity: Severity::High,
                    response: AlarmResponseTypes::File(FileArguments {
                        file_path: "~/test.sh".into(),
                        run_directory: Some("~/".into()),
                    }),
                    repeating: Some(Duration::from_secs(1)),
                },
            ],
        }],
    };

    let contents = serde_yaml::to_string(&config)?;

    std::fs::write(path, contents)?;

    Ok(())
}
