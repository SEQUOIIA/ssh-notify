use super::agent::{Discord, Email};

#[derive(Debug, Deserialize, Serialize)]
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
}