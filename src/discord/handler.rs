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

            let index = tymap.get::<IndexKey>().unwrap();
            if let Err(err) = index.guilds_update_online(data.guilds
                .iter()
                .map(|guild| guild.id().into())) {
                log::error!("Error updating online guilds: {}", err);
            }
    }

    fn guild_create(&self, ctx: Context, guild: model::Guild, is_new: bool) {
        if !is_new {
            return;
        }

        log::info!("Joined guild {} in {}", &guild.name, &guild.region);

        let tymap = ctx.data.read();
        let index = tymap.get::<IndexKey>().unwrap();
        if let Err(err) = index.guilds_new_join(GuildId::from(guild.id), &guild.name) {
            log::error!("Error registering new guild: {}", err);
        }
    }

    fn guild_member_removal(&self, ctx: Context, guild: model::GuildId, user: model::User, _member_data: Option<model::Member>) {
        let tymap = ctx.data.read();
        let secrets = tymap.get::<SecretsKey>().unwrap();
        let client_id = secrets.discord().client_id();

        if client_id == user.id.as_u64() {
            log::info!("Kicked from guild {}", guild.as_u64());
            let index = tymap.get::<IndexKey>().unwrap();
            if let Err(err) = index.guilds_kicked(guild.into()) {
                log::error!("Error unregistering kicked guild: {}", err);
            }
        }
    }
}
