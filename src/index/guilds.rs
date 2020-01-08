use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
use webcord_schema::models;
use webcord_schema::schema::guilds::dsl as guilds;

use super::{Index, QueryError};
use crate::GuildId;

impl Index {
    pub fn list_guilds(&self) -> Result<Vec<(GuildId, String)>, QueryError> {
        let guilds = guilds::guilds
            .select((guilds::id, guilds::cache_name))
            .filter(guilds::listed.eq(true))
            .load(&self.0.get()?)?;
        Ok(guilds
            .into_iter()
            .collect())
    }

    pub fn filter_enabled(
        &self,
        list: impl IntoIterator<Item = GuildId>,
    ) -> Result<Vec<FilterResult>, QueryError> {
        let guilds = guilds::guilds
            .select((guilds::id, guilds::listed))
            .filter(guilds::id.eq_any(list))
            .load::<(GuildId, bool)>(&self.0.get()?)?;
        Ok(guilds
            .into_iter()
            .map(|(guild_id, listed)| FilterResult { guild_id, listed })
            .collect())
    }

    pub fn new_join(&self, id: GuildId, name: &str) -> Result<(), QueryError> {
        diesel::insert_into(guilds::guilds)
            .values(&[models::Guild::new(id, name.to_string(), false)][..])
            .execute(&self.0.get()?)?;
        Ok(())
    }
}

pub struct FilterResult {
    pub guild_id: GuildId,
    pub listed: bool,
}
