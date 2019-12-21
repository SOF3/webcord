use crate::GuildInfo;

impl super::Bridge {
    pub async fn guild_info(&self, guild_id: u64) -> super::Result<GuildInfo> {
        self.index.guild_info(guild_id as i64)?;
        unimplemented!()
    }
}
