use super::agent::{Discord, Email};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "args")]
pub enum Agent {
    None,
    Email(Email),
    Discord(Discord),
}

impl Agent {
    fn send(&self, vars : Vars) -> Result<(), String> {
        match self {
            Agent::Email(v) => {
                v.run(vars);
                unimplemented!()
            }
            _ => {}
        }
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct Vars {
    pub user : String,
    pub r_user : String,
    pub r_host : String
}