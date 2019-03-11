use super::agent::{Discord, Email};
use std::net::{Ipv4Addr, IpAddr, Ipv6Addr, ToSocketAddrs};
use ipnet::{Ipv4Net, Ipv6Net};
use std::str::FromStr;

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
    pub is_whitelisted : bool,
}

pub struct Address(IpAddr);

impl Address {
    pub fn new(val : String) -> Result<Self, String> {
        //v4
        let addr = Ipv4Addr::from_str(val.as_str());
        if let Ok(v) = addr {
            return Ok(Self(IpAddr::V4(v)));
        }

        //v6
        let addr = Ipv6Addr::from_str(val.as_str());
        if let Ok(v) = addr {
            return Ok(Self(IpAddr::V6(v)));
        }

        //dns lookup
        let addrs = format!("{}:22", val).to_socket_addrs();
        if let Ok(mut v) = addrs {
            let addr = v.next().unwrap();
            match addr.ip() {
                IpAddr::V4(ip) => return Ok(Self(IpAddr::V4(ip))),
                IpAddr::V6(ip) => return Ok(Self(IpAddr::V6(ip))),
            }
        }

        Err("Unable to parse host address".to_owned())
    }


    // TODO, if one of the IP types is not specified, ignore it.
    pub fn is_whitelisted(&self, networks : &super::config::WhitelistedNetwork) -> bool {
        match self.0 {
            IpAddr::V4(v) => {
                match networks.ipv4.as_ref() {
                    Some(v4_networks) => {
                        for network in v4_networks {
                            let parsed_network = Ipv4Net::from_str(network.as_str()).unwrap();
                            let is_whitelisted = parsed_network.contains(&v);
                            if is_whitelisted {
                                return is_whitelisted
                            }
                        }
                    },
                    None => return false
                }
            },

            IpAddr::V6(v) => {
                match networks.ipv6.as_ref() {
                    Some(v6_networks) => {
                        for network in v6_networks {
                            let parsed_network : Ipv6Net = Ipv6Net::from_str(network.as_str()).unwrap();
                            let is_whitelisted = parsed_network.contains(&v);
                            if is_whitelisted {
                                return is_whitelisted
                            }
                        }
                    },
                    None => return false
                }
            }
        }

        false
    }

    pub fn socket_addr(&self) -> IpAddr {
        self.0
    }

    pub fn socket_addr_as_ref(&self) -> &IpAddr {
        &self.0
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_agentenum_toml() {
        let agent : Agent = Agent::Discord(Discord { data: super::super::config::ConfigDiscord {webhook_url: None} });
        println!("{}", toml::to_string_pretty(&agent).unwrap());
    }

    // Testing with the following subnets
    // IPv4: 192.168.1.0/24
    // IPv6: fc00:1:2:3::/64
    #[test]
    fn test_address_whitelist() {
        let addr = Address::new("192.168.1.128".to_owned()).unwrap();
        let addr_bad = Address::new("192.168.2.48".to_owned()).unwrap();
        let v6addr = Address::new("fc00:1:2:3::1".to_owned()).unwrap();
        let v6addr_bad = Address::new("fc00:1:2:3613::1".to_owned()).unwrap();
        let whitelist = super::super::config::WhitelistedNetwork { ipv4: Some(vec!("192.168.1.0/24".to_owned())), ipv6: Some(vec!("fc00:1:2:3::/64".to_owned())) };
        assert!(addr.is_whitelisted(&whitelist));
        assert!(v6addr.is_whitelisted(&whitelist));

        assert_eq!(addr_bad.is_whitelisted(&whitelist), false);
        assert_eq!(v6addr_bad.is_whitelisted(&whitelist), false);
    }
}