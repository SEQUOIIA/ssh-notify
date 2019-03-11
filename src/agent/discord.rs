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

        let res = client.post(url.as_str())
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
        if let Some(agents) = conf.agents {
            for agent in agents {
                if let model::Agent::Discord(v) = agent {
                    v.run(model::Vars {
                        user: "user".to_string(),
                        r_host: "remote host".to_string(),
                        hostname: "test-host".to_string(),
                        pam_type: "connect".to_string(),
                        is_whitelisted: false
                    });
                    break;
                }
            }
        }
    }
}