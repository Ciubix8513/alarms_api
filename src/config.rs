use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub data: Vec<ConfigItem>,
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
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum AlarmResponseTypes {
    Sound,
    #[default]
    Log,
}

#[test]
fn serialization_test() {
    let conf = Config {
        data: vec![
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
                        response: AlarmResponseTypes::Sound,
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
