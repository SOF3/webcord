use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, getset::Getters, Deserialize)]
#[get = "pub"]
pub struct Secrets {
    discord: DiscordSecrets,
    web: WebSecrets,
    database: DatabaseSecrets,
}

impl Secrets {
    pub fn try_new() -> Result<Self, config::ConfigError> {
        let mut config = config::Config::new();
        config.merge(config::File::with_name("config"))?;
        config.merge(config::Environment::new())?;
        Ok(config.try_into()?)
    }
}

#[derive(Debug, Clone, getset::Getters, Deserialize)]
#[get = "pub"]
pub struct DiscordSecrets {
    client_id: u64,
    client_secret: String,
    token: String,
}

#[derive(Debug, Clone, getset::Getters, Deserialize)]
pub struct WebSecrets {
    #[get = "pub"]
    addr: std::net::SocketAddr,
    #[get = "pub"]
    domain: String,
    entropy: Entropy32, // 32 bytes of entropy encoded in hex
}

impl WebSecrets {
    pub fn entropy(&self) -> &[u8; 32] {
        &self.entropy.0
    }
}

#[derive(Debug, Clone, getset::Getters, Deserialize)]
#[get = "pub"]
pub struct DatabaseSecrets {
    url: String,
}

#[derive(Debug, Clone)]
struct Entropy32(pub [u8; 32]);

impl<'de> Deserialize<'de> for Entropy32 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        let s = String::deserialize(d)?;
        if s.len() != 64 {
            return Err(D::Error::custom("Length must be 64"))?;
        }
        let mut ret = [0; 32];
        hex::decode_to_slice(s, &mut ret[..]).map_err(D::Error::custom)?;
        Ok(Self(ret))
    }
}
