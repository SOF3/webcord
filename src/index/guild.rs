use derive_more::{Display, From};
use diesel::pg::PgConnection;
use diesel::prelude::{BelongingToDsl, OptionalExtension, QueryDsl, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use webcord_schema::models;
#[allow(unused_imports)]
use webcord_schema::schema::{
    channel_hours::dsl as channel_hours, channels::dsl as channels, guilds::dsl as guilds,
    known_invites::dsl as known_invites,
};

use super::{Index, QueryError};
use crate::{ChannelInfo, GuildId, GuildInfo, Secrets};

impl Index {
    pub fn guild_info(&self, id: GuildId) -> Result<Option<GuildInfo>, QueryError> {
        let guild = guilds::guilds
            .find(id)
            .first::<models::Guild>(&self.0.get()?)
            .optional()?;
        match guild {
            Some(guild) => {
                let channels = <models::Channel as BelongingToDsl>::belonging_to(&guild)
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
