#[derive(Debug, Clone, getset::Getters, serde::Deserialize)]
#[get = "pub(crate)"]
pub(crate) struct Secrets {
    discord: DiscordSecrets,
    web: WebSecrets,
}

impl Secrets {
    pub(crate) fn try_new() -> Result<Self, config::ConfigError> {
        let mut config = config::Config::new();
        config.merge(config::File::with_name("config"))?;
        config.merge(config::Environment::new())?;
        Ok(config.try_into()?)
    }
}

#[derive(Debug, Clone, getset::Getters, serde::Deserialize)]
#[get = "pub(crate)"]
pub(crate) struct DiscordSecrets {
    token: String,
}

#[derive(Debug, Clone, getset::Getters, serde::Deserialize)]
#[get = "pub(crate)"]
pub(crate) struct WebSecrets {
    addr: std::net::SocketAddr,
}
