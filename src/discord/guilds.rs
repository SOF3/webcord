use crate::GuildInfo;

impl super::Bridge {
    pub async fn guild_info(&self, _guild_id: u64) -> super::Result<crate::GuildInfo> {
        unimplemented!()
    }
}
