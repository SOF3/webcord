use serenity::model::prelude as smodel;

use crate::index::{ChannelInfo, GuildInfo};
use crate::GuildId;

impl super::Bridge {
    pub fn guild_info(&self, guild_id: GuildId, refresh: bool) -> super::Result<GuildInfo> {
        if !refresh {
            if let Some(info) = self.index.guild_info(guild_id)? {
                return Ok(info);
            }
        }

        let guild = smodel::Guild::get(self.http(), guild_id)?;
        let channels = guild
            .channels(self.http())?
            .into_iter()
            .map(|(_, ch)| {
                ChannelInfo::new(
                    ch.id.into(),
                    ch.name.clone(),
                    ch.topic.unwrap_or_else(String::new),
                )
            })
            .collect();
        let gi = GuildInfo::new(guild.id.into(), guild.name, channels);

        // TODO store to index

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

pub struct Message {}
