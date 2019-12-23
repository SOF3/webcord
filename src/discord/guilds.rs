use crate::GuildId;

impl super::Bridge {
    pub fn list_guilds(&self) -> super::Result<Vec<(GuildId, String)>> {
        Ok(self.index.list_guilds()?)
    }
}
