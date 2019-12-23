use derive_more::{Display, From};
use diesel::pg::PgConnection;
use diesel::prelude::{
    BelongingToDsl, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
};
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
