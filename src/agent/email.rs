use super::super::config;
use super::super::model;
use lettre::{SmtpClient, Transport};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Email {
    pub data : config::ConfigEmail
}

impl Email {
    pub fn run(&self, vars : model::Vars) {
        let msg = format!("SSH Login: {} from {} on {}", vars.user, vars.r_host, vars.hostname);
        let payload = lettre_email::EmailBuilder::new()
            .to(self.data.recepient.as_ref().unwrap().as_str())
            .from(self.data.sender.as_ref().unwrap().as_str())
            .subject(msg.as_str())
            .text(msg.as_str())
            .build()
            .unwrap();

        let mut mailer = SmtpClient::new_simple(self.data.smtp_host.as_ref().unwrap().as_str())
            .unwrap()
            .credentials(lettre::smtp::authentication::Credentials::new(
                self.data.smtp_auth_user.as_ref().unwrap().clone(),
                self.data.smtp_auth_pass.as_ref().unwrap().clone()
            ))
            .smtp_utf8(true)
            .authentication_mechanism(lettre::smtp::authentication::Mechanism::Login)
            .transport();

        mailer.send(payload.into()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_exe;

    #[test]
    fn test_agent_email() {
        let conf = config::config();
        if let Some(agents) = conf.agents {
            for agent in agents {
                if let model::Agent::Email(v) = agent {
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