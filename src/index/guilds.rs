use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl, OptionalExtension};
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

    pub fn guilds_new_join(&self, id: GuildId, name: &str) -> Result<(), QueryError> {
        let row = guilds::guilds.select(guilds::online).filter(guilds::id.eq(id.to_db())).first::<bool>(&self.0.get()?).optional()?;
        match row {
            Some(true) => {
                log::warn!("Live cache desynchronization: guild {}#{} is joined in database, but Discord resent guild_create event", id, name)
            },
            Some(false) => {
                diesel::update(guilds::guilds)
                    .set(guilds::online.eq(true))
                    .execute(&self.0.get()?)?;
            },
            None => {
                diesel::insert_into(guilds::guilds)
                    .values(&[models::Guild::new(id.to_db(), name.to_string(), true, false)][..])
                    .execute(&self.0.get()?)?;
            }
        }
        Ok(())
    }

    pub fn guilds_update_online(&self, guilds_iter: impl Iterator<Item = GuildId> + Clone) -> Result<(), QueryError> {
        let changed_rows = diesel::update(guilds::guilds)
            .set(guilds::online.eq(guilds::id.eq_any(guilds_iter.clone().map(|id| id.to_db()))))
            .execute(&self.0.get()?)?;
        if changed_rows > 0 {
            log::info!("Dead cache desynchronization: Invited to or kicked from {} previously known guilds", changed_rows);
        }

        let new_guilds = guilds::guilds
            .filter(guilds::id.ne_all(guilds_iter.clone().map(|id| id.to_db())))
            .count()
            .get_result::<i64>(&self.0.get()?)?;

        if new_guilds > 0 {
            log::info!("Dead desynchronization: Joined {} previously unknown guilds", new_guilds);
        }
        Ok(())
    }

    pub fn guilds_kicked(&self, id: GuildId) -> Result<(), QueryError> {
        diesel::update(guilds::guilds.filter(guilds::id.eq(id.to_db())))
            .set(guilds::online.eq(false))
            .execute(&self.0.get()?)?;
        Ok(())
    }
}

pub struct FilterResult {
    pub guild_id: GuildId,
    pub listed: bool,
}
