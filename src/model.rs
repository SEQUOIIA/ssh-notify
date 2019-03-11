use super::agent::{Discord, Email};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "args")]
pub enum Agent {
    None,
    Email(Email),
    Discord(Discord),
}

impl Agent {
    pub fn send(&self, vars : Vars) -> Result<(), String> {
        match self {
            Agent::Email(v) => {
                v.run(vars);
                Ok(())
            },
            Agent::Discord(v) => {
                v.run(vars);
                Ok(())
            }
            Agent::None => Ok(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vars {
    pub user : String,
    pub r_host : String,
    pub hostname : String,
    pub pam_type : String,
}

mod tests {
    use super::*;

    #[test]
    fn test_agentenum_toml() {
        let agent : Agent = Agent::Discord(Discord { data: super::super::config::ConfigDiscord {webhook_url: None} });
        println!("{}", toml::to_string_pretty(&agent).unwrap());
    }
}