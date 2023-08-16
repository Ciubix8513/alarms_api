use std::{
    error::Error,
    path::{Path, PathBuf},
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub hosts: Vec<ConfigItem>,
    pub api_key: String,
    pub ip_address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConfigItem {
    pub name: String,
    pub responses: Vec<SeverityItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SeverityItem {
    pub severity: Severity,
    pub response: AlarmResponseTypes,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum Severity {
    #[default]
    Low,
    Middle,
    High,
    Test,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum RepeatingSatus {
    #[default]
    NonRepeating,
    Repeating(u32),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FileArguments {
    pub path: PathBuf,
    pub repeating: RepeatingSatus,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub enum AlarmResponseTypes {
    Sound,
    #[default]
    Log,
    File(FileArguments),
}

#[test]
fn serialization_test() {
    let conf = Config {
        api_key: "123".to_string(),
        port: 5000,
        ip_address: "127.0.0.1".to_string(),
        hosts: vec![
            ConfigItem {
                name: "Test".to_string(),
                responses: vec![
                    SeverityItem {
                        severity: Severity::Low,
                        response: AlarmResponseTypes::Log,
                    },
                    SeverityItem {
                        severity: Severity::Middle,
                        response: AlarmResponseTypes::Log,
                    },
                    SeverityItem {
                        severity: Severity::High,
                        response: AlarmResponseTypes::File(FileArguments {
                            path: "~/Projects/ed/target/debug/ed".into(),
                            repeating: RepeatingSatus::Repeating(1000),
                        }),
                    },
                ],
            },
            ConfigItem {
                name: "Test1".to_string(),
                responses: vec![
                    SeverityItem {
                        severity: Severity::Low,
                        response: AlarmResponseTypes::Log,
                    },
                    SeverityItem {
                        severity: Severity::Middle,
                        response: AlarmResponseTypes::Log,
                    },
                    SeverityItem {
                        severity: Severity::High,
                        response: AlarmResponseTypes::Sound,
                    },
                ],
            },
            ConfigItem {
                name: "Test2".to_string(),
                responses: vec![
                    SeverityItem {
                        severity: Severity::Low,
                        response: AlarmResponseTypes::Log,
                    },
                    SeverityItem {
                        severity: Severity::Middle,
                        response: AlarmResponseTypes::Log,
                    },
                    SeverityItem {
                        severity: Severity::High,
                        response: AlarmResponseTypes::Sound,
                    },
                ],
            },
        ],
    };
    let s = serde_yaml::to_string(&conf).unwrap();
    println!("{s}");
}

pub fn prarse_config(path: PathBuf) -> Result<Config, Box<dyn Error>> {
    Ok::<Config, Box<dyn Error>>(serde_yaml::from_str(std::str::from_utf8(&std::fs::read(
        path,
    )?)?)?)
}
pub fn generate_default_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        ip_address: "127.0.0.1".to_string(),
        port: 5000,
        api_key: "123".to_string(),
        hosts: vec![ConfigItem {
            name: "Host".to_string(),
            responses: vec![
                SeverityItem {
                    severity: Severity::Low,
                    response: AlarmResponseTypes::Log,
                },
                SeverityItem {
                    severity: Severity::Middle,
                    response: AlarmResponseTypes::Sound,
                },
                SeverityItem {
                    severity: Severity::High,
                    response: AlarmResponseTypes::File(FileArguments {
                        path: "~/test.sh".into(),
                        repeating: RepeatingSatus::Repeating(1000),
                    }),
                },
            ],
        }],
    };

    let contents = serde_yaml::to_string(&config)?;

    std::fs::write(path, contents)?;

    Ok(())
}
