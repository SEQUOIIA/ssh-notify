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

fn main() {
    let conf = config::config();
    let vars = get_pam_vars();

    if let Some(agents) = conf.agents {
        for ag in agents {
            match ag {
                _ => {
                    ag.send(vars.clone()).unwrap();
                }
            }
        }
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