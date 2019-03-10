use super::super::config;
use super::super::model;

#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub data : config::ConfigEmail
}

impl Email {
    pub fn run(&self, vars : model::Vars) {

    }
}