use actix_web::{error, web, HttpResponse};

use super::template;

#[actix_web::get("/logs/{guild}")]
pub(super) async fn handler(
    tmpl: web::Data<template::Templates>,
    path: web::Path<(u64,)>,
) -> Result<HttpResponse, error::Error> {
    let (guild,) = path.into_inner();
    let name = get_guild_name(guild).await?;
    let channels = get_guild_channels(guild).await?;
    let data = template::GuildArgs {
        guild: template::Guild {
            id: guild,
            name: &name,
        },
        channels: &channels,
    };
    let rendered = tmpl.guild(
        &template::PageArgs {
            title: &name,
            description: &format!("Chat logs for the Discord guild \"{}\"", &name),
        },
        &data,
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}

async fn get_guild_name(_guild: u64) -> Result<String, error::Error> {
    unimplemented!()
}

async fn get_guild_channels(_guild: u64) -> Result<Vec<(u64, String)>, error::Error> {
    unimplemented!()
}
