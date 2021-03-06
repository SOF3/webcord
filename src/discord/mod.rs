dirmod::all!(default pub use; default dir pub);

use std::thread;

use serenity::framework::standard::{macros::*, CommandResult, StandardFramework};
use serenity::http::client::Http;
use serenity::model::prelude::{self as model, Message};
use serenity::prelude::*;

use crate::index::Index;
use crate::Secrets;

pub struct Bridge {
    index: Index,
    http: Http,
}

impl Bridge {
    pub fn try_new(secrets: &Secrets, index: &Index) -> serenity::Result<Self> {
        let mut client = serenity::Client::new(secrets.discord().token(), handler::Handler)?;
        {
            let mut data = client.data.write();
            data.insert::<SecretsKey>(secrets.clone());
            data.insert::<IndexKey>(index.clone());
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
        Ok(Self {
            index: index.clone(),
            http: Http::new_with_token(secrets.discord().token()),
        })
    }

    fn http(&self) -> &Http {
        &self.http
    }
}

#[group]
#[commands(help)]
struct General;

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    log::info!("Received help command");

    let reply = {
        let tymap = ctx.data.read();
        let secrets = tymap.get::<SecretsKey>().unwrap();
        format!(
            "Mirroring this server at {domain}/guilds/{guild} live.\n\
             Invite this bot to your server: {domain}/invite",
            domain = secrets.web().domain(),
            guild = msg.guild_id.map_or(0, |id| *id.as_u64()),
        )
    };

    msg.reply(ctx, reply)?;

    Ok(())
}

struct SecretsKey;
impl typemap::Key for SecretsKey {
    type Value = Secrets;
}

struct IndexKey;
impl typemap::Key for IndexKey {
    type Value = Index;
}
