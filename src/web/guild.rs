use actix_web::{web, HttpResponse};

use super::{html, UserResult};
use crate::{block, discord, GuildId};

#[actix_web::get("/guilds/{guild}")]
pub(super) async fn handler(
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    path: web::Path<(GuildId,)>,
) -> UserResult<HttpResponse> {
    let (guild_id,) = path.into_inner();
    let guild = block(move || bridge.guild_info(guild_id, false))
        .await
        .map_err(global.priv_error("Error querying Discord API"))?;
    let rendered = html::guild::render(html::Args {
        global: global.as_ref(),
        page: html::PageArgs {
            title: guild.name(),
            description: &format!("Chat logs for the Discord guild \"{}\"", guild.name()),
        },
        local: html::guild::Local {
            guild: html::guild::Guild {
                id: guild_id,
                name: guild.name(),
            },
            channels: &guild
                .channels()
                .iter()
                .map(|ch| (ch.id(), ch.name().as_str()))
                .collect(),
        },
    });
    Ok(HttpResponse::Ok().body(rendered))
}
