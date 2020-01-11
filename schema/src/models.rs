// use derive_more::Display;
use derive_new::new;
// use diesel::deserialize::{FromSql, Queryable};
// use diesel::expression::AsExpression;
// use diesel::sql_types::BigInt;
use getset::{CopyGetters, Getters};

use crate::schema::*;

#[derive(Associations, Identifiable, Queryable, new, Getters, CopyGetters)]
#[belongs_to(Channel)]
#[primary_key(channel_id, date, hour)]
pub struct ChannelHour {
    #[get_copy = "pub"]
    channel_id: ChannelId,
    #[get = "pub"]
    date: chrono::Date<chrono::offset::Utc>,
    #[get_copy = "pub"]
    hour: u32,
    #[get_copy = "pub"]
    message: MessageId,
}

#[derive(Associations, Identifiable, Queryable, new, Getters, CopyGetters)]
#[belongs_to(Guild)]
pub struct Channel {
    #[get_copy = "pub"]
    id: ChannelId,
    #[get_copy = "pub"]
    guild_id: GuildId,
    #[get = "pub"]
    cache_name: String,
    #[get = "pub"]
    cache_desc: String,
}

#[derive(Identifiable, Insertable, Queryable, new, Getters, CopyGetters)]
pub struct Guild {
    #[get_copy = "pub"]
    id: GuildId,
    #[get = "pub"]
    cache_name: String,
    #[get = "pub"]
    online: bool,
    #[get_copy = "pub"]
    listed: bool,
}

#[derive(Associations, Identifiable, Queryable, Getters, CopyGetters)]
#[belongs_to(Guild)]
#[primary_key(code)]
pub struct KnownInvite {
    #[get = "pub"]
    code: String,
    #[get_copy = "pub"]
    guild_id: GuildId,
}

macro_rules! snowflake {
    ($($name:ident)*) => {$(
        type $name = i64;
    )*};
}
snowflake!(GuildId ChannelId MessageId);
