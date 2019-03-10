use std::fs::{OpenOptions};
use std::io::{Read};
use std::env::current_exe;
use log::{error};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub log : Option<bool>,
    pub log_path : Option<String>,
    pub agents : Option<Agents>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Agents {
    pub discord : Option<Vec<ConfigDiscord>>,
    pub email : Option<Vec<ConfigEmail>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigDiscord {
    pub webhook_url : Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigEmail {
    pub smtp_auth_user : Option<String>,
    pub smtp_auth_pass : Option<String>,
    pub smtp_host : Option<String>,
    pub recepient : Option<String>,
    pub sender : Option<String>
}

pub fn config() -> Config {
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("config.toml").unwrap();

    let mut buf = Vec::new();
    config_file.read_to_end(&mut buf).expect("Something went wrong while reading config file");
    let mut config : Config = toml::from_slice(&buf).unwrap();

    if let Some(v) = config.log {
        if v {
            if let None = config.log_path {
                let path = current_exe();
                match path {
                    Ok(p) => {
                        let path_dir = p.parent().unwrap().join("log").to_str().unwrap().to_owned();
                        config.log_path = Some(format!("{}", path_dir));
                    },
                    Err(_) => {
                        error!("Unable to get path of currently running binary. Since no log path has been specified, logging will be disabled.");
                        config.log = Some(false);
                    }
                }
            }
        }
    }

    config
}