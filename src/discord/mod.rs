use std::thread;

use serenity::framework::standard::{macros::*, CommandResult, StandardFramework};
use serenity::model::prelude::{self as model, Message};
use serenity::prelude::*;

use crate::Secrets;

pub struct Bridge {}

impl Bridge {
    pub(crate) fn try_new(secrets: &Secrets) -> serenity::Result<Self> {
        let mut client = serenity::Client::new(secrets.discord().token(), Handler)?;
        {
            client.data.write().insert::<SecretsKey>(secrets.clone());
        }
        client.with_framework(
            StandardFramework::new()
                .configure(|c| {
                    c.allow_dm(true)
                        .on_mention(Some(model::UserId(*secrets.discord().client_id())))
                })
                .group(&GENERAL_GROUP),
        );
        thread::spawn(move || {
            if let Err(err) = client.start_autosharded() {
                log::error!("Discord client error: {}", err);
            }
        });
        Ok(Self {})
    }
}

group!({
    name: "general",
    commands: [help],
});

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    log::info!("Received help command");

    let reply = {
        let tymap = ctx.data.read();
        let secrets = tymap.get::<SecretsKey>().unwrap();
        format!("Mirroring this server at {domain}/{guild} live.\n\
            Invite this bot to your server: {invite}",
            domain = secrets.web().domain(),
            guild = msg.guild_id.map_or(0, |id| *id.as_u64()),
            invite = invite_link(*secrets.discord().client_id()),
        )
    };

    msg.reply(ctx, reply)?;

    Ok(())
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data: model::Ready) {
        let tymap = ctx.data.read();
        let secrets = tymap.get::<SecretsKey>().unwrap();
        log::info!("Live on {} guilds", data.guilds.len());
        log::info!(
            "Invite link: {}",
            invite_link(*secrets.discord().client_id())
        );
        log::info!("Invite link: https://discordapp.com/api/oauth2/authorize?client_id={}&permissions=68608&scope=bot", secrets.discord().client_id());
        ctx.set_presence(
            Some(model::Activity::streaming(
                &format!("chat log on {}", secrets.web().domain()),
                secrets.web().domain(),
            )),
            model::OnlineStatus::Online,
        );
    }
}

struct SecretsKey;
impl typemap::Key for SecretsKey {
    type Value = Secrets;
}

pub fn invite_link<'a>(client_id: u64) -> String {
    format!(
        "https://discordapp.com/api/oauth2/authorize?client_id={}&permissions=68608&scope=bot",
        client_id
    )
}
