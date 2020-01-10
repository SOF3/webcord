#![allow(unused_variables)]

use std::sync::Arc;

use actix_web::{web, HttpResponse};

use super::{html, UserResult};
use crate::discord::{ChannelInfo, GuildInfo};
use crate::{block, discord, GuildId};

#[actix_web::get("/guilds/{guild}")]
pub(super) async fn handle_guild(
    login: super::Login,
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    path: web::Path<(GuildId,)>,
) -> UserResult<HttpResponse> {
    let (guild_id,) = path.into_inner();
    handler(login, bridge, global, guild_id, None, None).await
}

#[actix_web::get("/guilds/{guild}/{channel}")]
pub(super) async fn handle_channel(
    login: super::Login,
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    path: web::Path<(GuildId, String)>,
) -> UserResult<HttpResponse> {
    let (guild_id, channel_name) = path.into_inner();
    handler(login, bridge, global, guild_id, Some(channel_name), None).await
}

#[actix_web::get("/guilds/{guild}/{channel}/{year}/{month}/{date}")]
pub(super) async fn handle_date(
    login: super::Login,
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    path: web::Path<(GuildId, String, u32, u32, u32)>,
) -> UserResult<HttpResponse> {
    let (guild_id, channel_name, year, month, date) = path.into_inner();
    handler(
        login,
        bridge,
        global,
        guild_id,
        Some(channel_name),
        Some((year, month, date)),
    )
    .await
}

#[allow(unreachable_code)]
async fn handler(
    login: super::Login,
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    guild_id: GuildId,
    channel_name: Option<String>,
    ymd: Option<(u32, u32, u32)>,
) -> UserResult<HttpResponse> {
    let guild = {
        let bridge = Arc::clone(&bridge);
        block(move || bridge.guild_info(guild_id))
            .await
            .map_err(global.priv_error_code(404, "Guild does not exist"))?
    };

    if guild.channels().len() == 0 {
        let message = format!(
            "There are no channels visible to webcord in the server {}.",
            guild.name()
        );
        let rendered = html::error::render(
            &global,
            html::PageArgs {
                config: (),
                title: &format!("No channels in {}", guild.name()),
                description: &message,
                login: login.as_ref().map(|login| &login.disp),
            },
            html::error::Local { message: &message },
        )?;
        return Ok(HttpResponse::Ok().body(rendered));
    }

    let channel = match channel_name {
        Some(name) => named_channel_in(&guild, &name).await?,
        None => default_channel_in(&guild).await?,
    };

    let date = match ymd {
        Some((y, m, d)) => chrono::NaiveDate::from_ymd(y as i32, m, d),
        None => latest_ymd_in(&channel).await?,
    };

    let messages = block(move || bridge.fetch_messages(&channel, date))
        .await
        .map_err(global.priv_error("Failed to fetch message history"))?;

    let groups: Vec<html::channel::Group<'_, &[_]>> = unimplemented!();
    let current_group = &groups[0]; // TODO unimplemented
    let messages = messages.into_iter().map(|message| html::channel::Message {
        time: unimplemented!(),
        author: unimplemented!(),
        content: horrorshow::html!(), // TODO unimplemented
        reactions: std::iter::once(unimplemented!()),
    });

    let rendered: String = html::channel::render(&global, html::PageArgs {
        config: client::PageConfig,
        title: &format!("#{} | {} ({})", &channel.name(), guild.name(), date.format("%Y-%m-%d")),
        description: &format!("Message history on #{} of {} on {}. Open webcord on your browser to see a live mirror.", &channel.name(), guild.name(), date.format("%Y-%m-%d")),
        login: login.as_ref().map(|login| &login.disp),
    }, html::channel::Guild {
        id: guild.id(),
        name: guild.name(),
    },
        &groups,
        current_group,
        unimplemented!("current_channel"),
        date,
        messages,
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}

async fn default_channel_in(guild: &GuildInfo) -> UserResult<ChannelInfo> {
    unimplemented!()
}

async fn named_channel_in(guild: &GuildInfo, name: &str) -> UserResult<ChannelInfo> {
    unimplemented!()
}

async fn latest_ymd_in(channel: &ChannelInfo) -> UserResult<chrono::NaiveDate> {
    unimplemented!()
}

mod client {
    #[derive(serde::Serialize)]
    pub(super) struct PageConfig;

    impl crate::web::html::PageConfig for PageConfig {
        fn page_type() -> &'static str {
            "channel"
        }
    }
}
