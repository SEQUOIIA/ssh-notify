use super::super::config;
use super::super::model;

#[derive(Debug, Deserialize, Serialize)]
pub struct Discord {
    pub data : config::ConfigDiscord
}

impl Discord {
    pub fn run(&self, vars : model::Vars) {

    }
}