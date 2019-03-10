extern crate pretty_env_logger;
extern crate log;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
extern crate lettre;
extern crate lettre_email;

mod config;
mod model;
pub mod agent;

use std::env::{var};

fn main() {
    setup();
    let conf = config::config();
    let vars = get_pam_vars();
    println!("{:?}", vars);
    println!("{:?}", conf);
}


fn get_pam_vars() -> model::Vars {
    let user = var("PAM_USER").expect("PAM ENV(PAM_USER) not found");
    let ruser = var("PAM_RUSER").expect("PAM ENV(PAM_RUSER) not found");
    let rhost = var("PAM_RHOST").expect("PAM ENV(PAM_RHOST) not found");

    model::Vars {user, r_user: ruser, r_host: rhost}
}

#[cfg(debug_assertions)]
fn setup() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
}

#[cfg(not(debug_assertions))]
fn setup() {

}