use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
use webcord_schema::models;
use webcord_schema::schema::guilds::dsl as guilds;

use super::{Index, QueryError};
use crate::GuildId;

impl Index {
    pub fn list_guilds(&self) -> Result<Vec<(GuildId, String)>, QueryError> {
        let guilds = guilds::guilds
            .filter(guilds::listed.eq(true))
            .load::<models::Guild>(&self.0.get()?)?;
        Ok(guilds
            .into_iter()
            .map(|guild| (guild.id(), guild.into_name()))
            .collect())
    }
}
