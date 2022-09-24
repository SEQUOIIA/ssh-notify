extern crate pretty_env_logger;
extern crate log;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
extern crate lettre;
extern crate lettre_email;
extern crate hostname;
extern crate ipnet;

mod config;
mod model;
pub mod agent;

use std::env::{var};
use std::env::current_exe;
use std::fs::{OpenOptions};
use std::process::Command;
use log::{info};

fn main() {
    setup();
    let conf = config::config();
    let mut vars = get_pam_vars();

    if let Some(v) = conf.whitelisted_network.as_ref() {
        let addr = model::Address::new(vars.r_host.clone());

        if let Ok(a) = addr {
            vars.is_whitelisted = a.is_whitelisted(v);
        }
    }

    if !vars.pam_type.contains("close_session") && !vars.is_whitelisted {
        info!(target: "SSH-LOGIN", "{} from {} on {}", vars.user, vars.r_host, vars.hostname);

        let path = current_exe().unwrap();
        let providers_child_path = path.parent().unwrap().join("ssh-notify-agent");

        let _providers_child = Command::new(providers_child_path)
            .env("PAM_USER", vars.user)
            .env("PAM_RHOST", vars.r_host)
            .env("PAM_TYPE", vars.pam_type)
            .spawn()
            .expect("Failed to exec providers_child");

    }
}


fn get_pam_vars() -> model::Vars {
    let user = var("PAM_USER").expect("PAM ENV(PAM_USER) not found");
    let rhost = var("PAM_RHOST").expect("PAM ENV(PAM_RHOST) not found");
    let pamtype = var("PAM_TYPE").expect("PAM ENV(PAM_TYPE) not found");
    let hostname_v = hostname::get().unwrap().into_string().unwrap();
    let is_whitelisted : bool = false;

    model::Vars {user, r_host: rhost, hostname: hostname_v, pam_type: pamtype, is_whitelisted}
}

fn setup() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
}