impl super::Bridge {
    pub async fn guild_info(&self, _guild_id: u64) -> super::Result<GuildInfo> {
        unimplemented!()
    }
}

#[derive(getset::Getters)]
pub struct GuildInfo {
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    channels: Vec<(u64, String)>,
}
