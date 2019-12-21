use derive_more::From;
use diesel::pg::PgConnection;
use diesel::prelude::{BelongingToDsl, OptionalExtension, QueryDsl, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use webcord_schema::models;
use webcord_schema::schema::{
    channel_hours::dsl as channel_hours, channels::dsl as channels, guilds::dsl as guilds,
    known_invites::dsl as known_invites,
};

use crate::{ChannelInfo, GuildInfo, Secrets};

type ConnMan = ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ConnMan>;

#[derive(Clone)]
pub(crate) struct Index(Pool);

impl Index {
    pub(crate) fn try_new(secrets: &Secrets) -> Result<Index, r2d2::Error> {
        Ok(Index(Pool::new(ConnMan::new(secrets.database().url()))?))
    }

    pub(crate) fn guild_info(&self, id: models::GuildId) -> Result<Option<GuildInfo>, QueryError> {
        let guild = guilds::guilds
            .find(id)
            .first::<models::Guild>(&self.0.get()?)
            .optional()?;
        match guild {
            Some(guild) => {
                let channels = models::Channel::belonging_to(&guild)
                    .load::<models::Channel>(&self.0.get()?)?;
                let channels = channels
                    .into_iter()
                    .map(
                        |ch| ChannelInfo::new(ch.id(), ch.cache_name().clone(), ch.cache_desc().clone()),
                    )
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

#[derive(From)]
pub(crate) enum QueryError {
    R2d2(r2d2::Error),
    Diesel(diesel::result::Error),
}
