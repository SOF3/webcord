#![allow(unused_imports)]
dirmod::all!();

use derive_more::{Display, From};
use diesel::pg::PgConnection;
use diesel::prelude::{BelongingToDsl, OptionalExtension, QueryDsl, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use webcord_schema::models;
use webcord_schema::schema::{
    channel_hours::dsl as channel_hours, channels::dsl as channels,
    known_invites::dsl as known_invites,
};

use crate::{ChannelInfo, GuildInfo, Secrets};

type ConnMan = ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ConnMan>;

#[derive(Clone)]
pub struct Index(Pool);

impl Index {
    pub fn try_new(secrets: &Secrets) -> Result<Index, r2d2::Error> {
        Ok(Index(Pool::new(ConnMan::new(secrets.database().url()))?))
    }
}

#[derive(From, Display)]
pub enum QueryError {
    R2d2(r2d2::Error),
    Diesel(diesel::result::Error),
}
