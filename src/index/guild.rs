use crate::{ChannelInfo, GuildInfo};
use diesel::prelude::{BelongingToDsl, OptionalExtension, QueryDsl, RunQueryDsl};
use webcord_schema::models;
use webcord_schema::schema::guilds::dsl as guilds;

use super::{Index, QueryError};
use crate::GuildId;

impl Index {
    pub fn guild_info(&self, id: GuildId) -> Result<Option<GuildInfo>, QueryError> {
        let guild = guilds::guilds
            .find(id)
            .first::<models::Guild>(&self.0.get()?)
            .optional()?;
        match guild {
            Some(guild) => {
                let channels =
                    <models::Channel as BelongingToDsl<&models::Guild>>::belonging_to(&guild)
                        .load::<models::Channel>(&self.0.get()?)?;
                let channels = channels
                    .into_iter()
                    .map(|ch| {
                        ChannelInfo::new(ch.id(), ch.cache_name().clone(), ch.cache_desc().clone())
                    })
                    .collect();
                Ok(Some(GuildInfo::new(
                    id,
                    guild.cache_name().clone(),
                    channels,
                )))
            }
            None => Ok(None),
        }
    }
}
