use actix_web::{web, HttpResponse};

use super::{template, UserResult};
use crate::{block, discord};

#[actix_web::get("/guilds")]
pub(super) async fn handler(
    bridge: web::Data<discord::Bridge>,
    tmpl: web::Data<template::Templates>,
) -> UserResult<HttpResponse> {
    let guilds = block(move || bridge.list_guilds())
        .await
        .map_err(tmpl.clone().priv_error("Error querying Discord API"))?;
    let count = guilds.len();
    let data = template::GuildsArgs {
        guilds: &guilds
            .iter()
            .map(|(id, name)| (*id, name.as_str()))
            .collect(),
    };
    let rendered = tmpl.guilds(
        &template::PageArgs {
            title: "Guilds mirrored by webcord",
            description: &format!("webcord is mirroring chat from {} guilds", count),
        },
        &data,
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}
