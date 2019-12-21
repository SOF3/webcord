use actix_web::{web, HttpResponse};

use super::{template, UserResult};
use crate::discord;

#[actix_web::get("/logs/{guild}")]
pub(super) async fn handler(
    bridge: web::Data<discord::Bridge>,
    tmpl: web::Data<template::Templates>,
    path: web::Path<(u64,)>,
) -> UserResult<HttpResponse> {
    let tmpl = tmpl.into_inner();
    let (guild_id,) = path.into_inner();
    let guild = bridge
        .guild_info(guild_id)
        .await
        .map_err(tmpl.clone().priv_error("Error querying Discord API"))?;
    let data = template::GuildArgs {
        guild: template::Guild {
            id: guild_id,
            name: guild.name(),
        },
        channels: unimplemented!(),
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
