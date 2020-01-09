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
            .load::<(i64, String)>(&self.0.get()?)?;
        Ok(guilds
            .into_iter()
            .map(|(guild_id, name)| (guild_id.into(), name))
            .collect())
    }

    pub fn filter_enabled(
        &self,
        list: impl IntoIterator<Item = GuildId>,
    ) -> Result<Vec<FilterResult>, QueryError> {
        let guilds = guilds::guilds
            .select((guilds::id, guilds::listed))
            .filter(guilds::id.eq_any(list.into_iter().map(|id| id.to_db())))
            .load::<(i64, bool)>(&self.0.get()?)?;
        Ok(guilds
            .into_iter()
            .map(|(guild_id, listed)| FilterResult {
                guild_id: guild_id.into(),
                listed,
            })
            .collect())
    }

    pub fn new_join(&self, id: GuildId, name: &str) -> Result<(), QueryError> {
        diesel::insert_into(guilds::guilds)
            .values(&[models::Guild::new(id.to_db(), name.to_string(), false)][..])
            .execute(&self.0.get()?)?;
        Ok(())
    }
}

pub struct FilterResult {
    pub guild_id: GuildId,
    pub listed: bool,
}
