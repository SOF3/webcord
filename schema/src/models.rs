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

#[derive(Identifiable, Queryable, new, Getters, CopyGetters)]
pub struct Guild {
    #[get_copy = "pub"]
    id: GuildId,
    #[get = "pub"]
    cache_name: String,
    #[get_copy = "pub"]
    listed: bool,
}

impl Guild {
    pub fn into_name(self) -> String {
        self.cache_name
    }
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

#[allow(unused_macros)] // TODO fix impl problems and switch to newtype
macro_rules! snowflake {
    ($($name:ident)*) => {$(
        #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
        pub struct $name(pub u64);

        impl AsExpression<BigInt> for $name {
            type Expression = <i64 as AsExpression<BigInt>>::Expression;

            fn as_expression(self) -> Self::Expression {
                AsExpression::<BigInt>::as_expression(self.0 as i64)
            }
        }

        impl<ST, DB: diesel::backend::Backend> Queryable<ST, DB> for $name
        where i64: FromSql<ST, DB> {
            type Row = <i64 as Queryable<ST, DB>>::Row;

            fn build(row: Self::Row) -> Self {
                let inner = <i64 as Queryable<ST, DB>>::build(row);
                Self(inner as u64)
            }
        }
    )*};
}
macro_rules! snowflake2 {
    ($($name:ident)*) => {$(
        pub type $name = i64;
    )*};
}
snowflake2!(GuildId ChannelId MessageId);
