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
#[macro_use]
extern crate log4rs;

mod config;
mod model;
pub mod agent;

use std::env::{var};
use std::env::current_exe;
use std::fs::{OpenOptions};

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
}


fn get_pam_vars() -> model::Vars {
    let user = var("PAM_USER").expect("PAM ENV(PAM_USER) not found");
    let rhost = var("PAM_RHOST").expect("PAM ENV(PAM_RHOST) not found");
    let pamtype = var("PAM_TYPE").expect("PAM ENV(PAM_RHOST) not found");
    let hostname_v = hostname::get_hostname().unwrap();
    let is_whitelisted : bool = false;

    model::Vars {user, r_host: rhost, hostname: hostname_v, pam_type: pamtype, is_whitelisted}
}

#[cfg(debug_assertions)]
fn setup() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
}

#[cfg(not(debug_assertions))]
fn setup() {
    std::env::set_var("RUST_LOG", "info");
    let path = current_exe().unwrap();
    let log_path = path.parent().unwrap().join("ssh_notify-log");

    let logfile = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{l} - {m}\n")))
        .build(log_path).unwrap();

    let conf = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("logfile", Box::new(logfile)))
        .build(log4rs::config::Root::builder()
            .appender("logfile")
            .build(log::LevelFilter::Info)).unwrap();

    log4rs::init_config(conf).unwrap();
}