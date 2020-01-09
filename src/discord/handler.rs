use serenity::model::prelude::{self as model};
use serenity::prelude::*;

use super::{IndexKey, SecretsKey};
use crate::GuildId;

pub(super) struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data: model::Ready) {
        let tymap = ctx.data.read();
        let secrets = tymap.get::<SecretsKey>().unwrap();
        log::info!("Live on {} guilds", data.guilds.len());
        ctx.set_presence(
            Some(model::Activity::streaming(
                &format!("chat log on {}", secrets.web().domain()),
                secrets.web().domain(),
            )),
            model::OnlineStatus::Online,
        );
    }

    fn guild_create(&self, ctx: Context, guild: model::Guild, is_new: bool) {
        if !is_new {
            return;
        }

        log::info!("Joined guild {} in {}", &guild.name, &guild.region);

        let tymap = ctx.data.read();
        let index = tymap.get::<IndexKey>().unwrap();
        if let Err(err) = index.new_join(GuildId::from(guild.id), &guild.name) {
            log::error!("Error registering new guild: {}", err);
        }

        // TODO store channels
    }

    // TODO store new channels
}
