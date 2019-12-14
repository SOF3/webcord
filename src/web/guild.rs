use actix_web::{error, web, HttpResponse};

use super::template;
use crate::discord;

#[actix_web::get("/logs/{guild}")]
pub(super) async fn handler(
    bridge: web::Data<discord::Bridge>,
    tmpl: web::Data<template::Templates>,
    path: web::Path<(u64,)>,
) -> Result<HttpResponse, error::Error> {
    let (guild_id,) = path.into_inner();
    let guild = bridge
        .guild_info(guild_id)
        .await
        .map_err(super::internal_error("Error querying Discord API"))?;
    let data = template::GuildArgs {
        guild: template::Guild {
            id: guild_id,
            name: guild.name(),
        },
        channels: guild.channels(),
    };
    let rendered = tmpl.guild(
        &template::PageArgs {
            title: guild.name(),
            description: &format!("Chat logs for the Discord guild \"{}\"", guild.name()),
        },
        &data,
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}
