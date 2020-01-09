use std::str::FromStr;

use horrorshow::{RenderOnce, TemplateBuffer};
use serde::{de::Error, Deserialize, Deserializer};

pub(crate) trait Snowflake: Copy {
    fn from_raw(u64: u64) -> Self;

    fn to_raw(self) -> u64;
}

macro_rules! make_id {
    ($($id:ident $(.$s:ident)?)*) => {$(
        /// A glue type that connects different representations of the same ID.
        ///
        /// - Serenity from/into
        /// - u64 from/into
        /// - horrorshow rendering (displayed as u64, for use in links
        #[derive(
            Debug, derive_more::Display,
            Clone, Copy,
            PartialEq, Eq, Hash,
        )]
        pub struct $id(u64);

        impl $id {
            #[allow(dead_code)]
            pub fn to_db(self) -> i64 { self.to_raw() as i64}
        }

        impl Snowflake for $id {
            fn from_raw(u64: u64) -> Self {
                Self(u64)
            }

            fn to_raw(self) -> u64 {
                self.0
            }
        }

        impl<'de> Deserialize<'de> for $id {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum Either<'d> {
                    S(&'d str),
                    U(u64),
                }

                let sou = Either::<'de>::deserialize(d)?;
                let u64 = match sou {
                    Either::S(s) => s.parse::<u64>().map_err(|err| D::Error::custom(err))?,
                    Either::U(u) => u,
                };
                Ok(Self::from_raw(u64))
            }
        }

        /// Converts the ID from its database representation.
        impl From<i64> for $id {
            fn from(i: i64) -> Self {
                Self(i as u64)
            }
        }

        /*impl Deref for $id {
            type Target = u64;

            fn deref(&self) -> &u64 { &self.0 }
        }*/

        impl FromStr for $id {
            type Err = <u64 as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <u64 as FromStr>::from_str(s)
                    .map(|u| Self(u))
            }
        }

        /// Renders the snowflake ID as
        impl RenderOnce for $id {
            fn render_once(self, tmpl: &mut TemplateBuffer) {
                <u64 as RenderOnce>::render_once(self.0, tmpl)
            }
        }

        $(
            noop!($s);

            impl From<$id> for ::serenity::model::id::$id {
                fn from(id: $id) -> Self {
                    Self::from(id.0)
                }
            }

            impl From<::serenity::model::id::$id> for $id {
                fn from(id: ::serenity::model::id::$id) -> $id {
                    $id(u64::from(id))
                }
            }
        )?
    )*};
}

pub(crate) mod id_str {
    use serde::Serializer;

    use super::Snowflake;

    pub(crate) fn serialize<S: Serializer>(
        id: &impl Snowflake,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&format_args!("{}", id.to_raw()))
    }
}

#[allow(dead_code)]
pub(crate) mod id_raw {
    use serde::Serializer;

    use super::Snowflake;

    pub(crate) fn serialize<S: Serializer>(
        id: &impl Snowflake,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(id.to_raw())
    }
}

make_id!(GuildId.s ChannelId.s CategoryId MessageId.s UserId.s EmojiId.s);
