use std::fs::{OpenOptions};
use std::io::{Read};
use std::env::current_exe;
use log::{error};
use super::model::{Agent};
use super::agent::{Discord, Email};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub log : Option<bool>,
    pub log_path : Option<String>,
    pub agents: Option<Vec<Agent>>,
    pub whitelisted_network: Option<WhitelistedNetwork>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WhitelistedNetwork {
    pub ipv4: Option<Vec<String>>,
    pub ipv6: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Agents {
    pub discord : Option<Vec<ConfigDiscord>>,
    pub email : Option<Vec<ConfigEmail>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigDiscord {
    pub webhook_url : Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigEmail {
    pub smtp_auth_user : Option<String>,
    pub smtp_auth_pass : Option<String>,
    pub smtp_host : Option<String>,
    pub recepient : Option<String>,
    pub sender : Option<String>
}

pub fn config() -> Config {
    let path = current_exe().unwrap();
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.parent().unwrap().join("config.toml")).unwrap();

    let mut buf = Vec::new();
    config_file.read_to_end(&mut buf).expect("Something went wrong while reading config file");
    let mut config : Config = toml::from_slice(&buf).unwrap();

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_toml() {
        let conf = Config {
            log: Some(true),
            log_path: None,
            whitelisted_network: None,
            agents: Some(vec!(Agent::None,
                                  Agent::Discord(Discord {data: ConfigDiscord {webhook_url: Some("Wowza".to_owned())}}),
                                  Agent::Email(Email {data: ConfigEmail {
                                      smtp_auth_pass: Some("test".to_owned()),
                                      smtp_auth_user: None,
                                      smtp_host: None,
                                      recepient: None,
                                      sender: None
                                  }})
            )
            ),
        };

        println!("{}", toml::to_string_pretty(&conf).unwrap());

    }
}