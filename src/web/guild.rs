use actix_web::{web, HttpResponse};

use super::{template, UserResult};
use crate::{block, discord, GuildId};

#[actix_web::get("/logs/{guild}")]
pub(super) async fn handler(
    bridge: web::Data<discord::Bridge>,
    tmpl: web::Data<template::Templates>,
    path: web::Path<(GuildId,)>,
) -> UserResult<HttpResponse> {
    let tmpl = tmpl.into_inner();
    let (guild_id,) = path.into_inner();
    let guild = block(move || bridge.guild_info(guild_id, false))
        .await
        .map_err(tmpl.clone().priv_error("Error querying Discord API"))?;
    let data = template::GuildArgs {
        guild: template::Guild {
            id: guild_id,
            name: guild.name(),
        },
        channels: guild
            .channels()
            .iter()
            .map(|ch| (ch.id(), ch.name().as_str()))
            .collect(),
    };
    let rendered = tmpl.clone().guild(
        &template::PageArgs {
            title: guild.name(),
            description: &format!("Chat logs for the Discord guild \"{}\"", guild.name()),
        },
        &data,
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}
