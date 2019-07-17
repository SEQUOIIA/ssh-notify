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

#[cfg(feature = "syslog")]
#[cfg(target_os = "linux")]
extern crate log4rs_syslog;

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
    if conf.log.as_ref().unwrap().clone() {
        if let Some(path) = conf.log_path.as_ref() {
            logging(Some(path));
        } else {
            logging(None);
        }
    }
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
fn setup() {}

#[cfg(debug_assertions)]
fn logging(path_v : Option<&str>) {

}

#[cfg(not(debug_assertions))]
fn logging(path_v : Option<&str>) {
    let syslog_enable : bool = config::config().syslog_enable.as_ref().unwrap().clone();
    let path = current_exe().unwrap();
    let log_path = match path_v {
        Some(v) => std::path::Path::new(v).to_path_buf(),
        None => path.parent().unwrap().join("log/ssh_notify")
    };

    let logfile = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{l} | {t} :: {m}\n")))
        .build(log_path).unwrap();

    let mut conf = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("logfile", Box::new(logfile)));

    if cfg!(feature = "syslog") && syslog_enable {
        let encoder = Box::new(log4rs::encode::pattern::PatternEncoder::new("{M} - {m}"));
        let syslog_appender = Box::new(
            log4rs_syslog::SyslogAppender::builder()
            .encoder(encoder)
            .openlog(
                "ssh-notify",
                log4rs_syslog::LogOption::LOG_PID,
                log4rs_syslog::Facility::Daemon,
            )
            .build(),
        );

        conf = conf.appender(log4rs::config::Appender::builder().build(
            "syslog",
            syslog_appender
        ));
    }


    let root_builder;
    if cfg!(feature = "syslog") && syslog_enable {
        root_builder = log4rs::config::Root::builder()
            .appender("syslog")
            .appender("logfile")
            .build(log::LevelFilter::Off);
    } else {
        root_builder = log4rs::config::Root::builder()
            .appender("logfile")
            .build(log::LevelFilter::Off);    
    }
    
    let conf = conf
        .logger(log4rs::config::Logger::builder().build("SSH-LOGIN", log::LevelFilter::Info))
        .build(root_builder).unwrap();

    log4rs::init_config(conf).unwrap();
    
}