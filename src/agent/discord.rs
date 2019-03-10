use super::super::config;
use super::super::model;

#[derive(Debug, Deserialize, Serialize)]
pub struct Discord {
    pub data : config::ConfigDiscord
}

impl Discord {
    pub fn run(self, vars : model::Vars) {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();

        let url = self.data.webhook_url.unwrap();

        let mut res = client.post(url.as_str())
            .header("Content-Type", "application/json")
            .body(format!("{{\"content\": \"SSH Login: {} from {} on {}\"}}", vars.user, vars.r_host, ""))
            .send().unwrap();
    }
}