use derive_new::new;
use getset::{CopyGetters, Getters};
use serenity::model::prelude as smodel;

use crate::{ChannelId, GuildId};

impl super::Bridge {
    pub fn guild_info(&self, guild_id: GuildId) -> super::Result<GuildInfo> {
        let guild = smodel::Guild::get(self.http(), guild_id)?;
        let channels = guild
            .channels(self.http())?
            .into_iter()
            .map(|(_, ch)| {
                ChannelInfo::new(
                    ChannelId::from(ch.id),
                    ch.name.clone(),
                    ch.topic.unwrap_or_else(String::new),
                )
            })
            .collect();
        let gi = GuildInfo::new(guild.id.into(), guild.name, channels);

        Ok(gi)
    }

    pub fn fetch_messages(
        &self,
        _channel: &ChannelInfo,
        _date: chrono::NaiveDate,
    ) -> super::Result<Vec<Message>> {
        unimplemented!()
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

pub struct Message {}
