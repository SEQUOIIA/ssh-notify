use super::super::config;
use super::super::model;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Discord {
    pub data : config::ConfigDiscord
}

impl Discord {
    pub fn run(&self, vars : model::Vars) {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();

        let url = self.data.webhook_url.as_ref().unwrap();

        let mut res = client.post(url.as_str())
            .header("Content-Type", "application/json")
            .body(format!("{{\"content\": \"SSH Login: {} from {} on {}\"}}", vars.user, vars.r_host, vars.hostname))
            .send().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_discord() {
        let conf = config::config();
        let discconf = conf.agents.as_ref().unwrap().discord.as_ref().unwrap()[0].clone();
        let discord = Discord {data: discconf};

        discord.run(model::Vars {
            user: "user".to_string(),
            r_host: "remote host".to_string(),
            hostname: "test-host".to_string(),
            pam_type: "connect".to_string(),
        });
    }
}