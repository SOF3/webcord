use derive_new::new;
use diesel::prelude::{BelongingToDsl, OptionalExtension, QueryDsl, RunQueryDsl};
use getset::{CopyGetters, Getters};
use webcord_schema::models;
use webcord_schema::schema::guilds::dsl as guilds;

use super::{Index, QueryError};
use crate::{ChannelId, GuildId};

impl Index {
    pub fn guild_info(&self, id: GuildId) -> Result<Option<GuildInfo>, QueryError> {
        let id = id.to_db();

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
                        ChannelInfo::new(
                            ch.id().into(),
                            ch.cache_name().clone(),
                            ch.cache_desc().clone(),
                        )
                    })
                    .collect();
                Ok(Some(GuildInfo::new(
                    id.into(),
                    guild.cache_name().clone(),
                    channels,
                )))
            }
            None => Ok(None),
        }
    }
}

#[derive(Debug, CopyGetters, Getters, new)]
pub struct GuildInfo {
    #[get_copy = "pub"]
    id: GuildId,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    channels: Vec<ChannelInfo>,
}

#[derive(Debug, CopyGetters, Getters, new)]
pub struct ChannelInfo {
    #[get_copy = "pub"]
    id: ChannelId,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    description: String,
}
