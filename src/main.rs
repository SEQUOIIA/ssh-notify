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

mod config;
mod model;
pub mod agent;

use std::env::{var};

fn main() {
    setup();
    let conf = config::config();
    let vars = get_pam_vars();

    if let Some(agents) = conf.agents {
        if let Some(discords) = agents.discord {
            for discord in discords.iter() {
                let disc : model::Agent = model::Agent::Discord(agent::Discord {data: discord.clone()});
                disc.send(vars.clone());
            }
        }

        if let Some(emails) = agents.email {
            for email in emails.iter() {
                let email : model::Agent = model::Agent::Email(agent::Email {data: email.clone()});
                email.send(vars.clone());
            }
        }
    }
}


fn get_pam_vars() -> model::Vars {
    let user = var("PAM_USER").expect("PAM ENV(PAM_USER) not found");
    let ruser = var("PAM_RUSER").expect("PAM ENV(PAM_RUSER) not found");
    let rhost = var("PAM_RHOST").expect("PAM ENV(PAM_RHOST) not found");
    let hostname_v = hostname::get_hostname().unwrap();

    model::Vars {user, r_user: ruser, r_host: rhost, hostname: hostname_v}
}

#[cfg(debug_assertions)]
fn setup() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
}

#[cfg(not(debug_assertions))]
fn setup() {
    std::env::set_var("RUST_LOG", "trace");
}