use std::collections::HashMap;

use actix_web::{web, HttpResponse};

use super::{html, UserResult};
use crate::index::Index;
use crate::GuildId;

#[actix_web::get("/account")]
pub(super) async fn handler(
    login: super::Login,
    global: web::Data<html::GlobalArgs>,
    common_client: web::Data<reqwest::Client>,
    index: web::Data<Index>,
) -> UserResult<HttpResponse> {
    let login = match &*login {
        Some(login) => login,
        None => return Ok(super::redirect("/")),
    };

    #[derive(Debug, serde::Deserialize)]
    struct PartialGuild {
        id: String,
        name: String,
        icon: Option<String>,
        // owner: bool,
        permissions: u64,
    }
    let guilds = common_client
        .get("https://discordapp.com/api/users/@me/guilds")
        .bearer_auth(&login.token)
        .send()
        .await
        .map_err(global.priv_error("Error loading guild list"))?
        .json::<Vec<PartialGuild>>()
        .await
        .map_err(global.priv_error("Error loading guild list"))?;

    let with_admin = guilds
        .iter()
        .filter(|guild| guild.permissions & 8 == 8)
        .map(|guild| (guild.id.parse::<u64>().unwrap_or(0) as GuildId, guild))
        .collect::<HashMap<_, _>>();

    let mut enabled = index
        .filter_enabled(with_admin.keys().copied())
        .map_err(global.priv_error("Error loading guild configuration"))?
        .into_iter()
        .map(|guild| {
            let object = &with_admin[&guild.guild_id];
            html::account::GuildEntry {
                id: guild.guild_id, // ignored error
                name: &object.name,
                icon: object.icon.as_ref().map(|s| s.as_str()),
                listed: guild.listed,
            }
        });

    let rendered = html::account::render(html::Args {
        global: global.as_ref(),
        page: html::PageArgs {
            title: "Manage account",
            description: "Manage guilds under your account",
            login: Some(&login.disp),
        },
        local: html::account::Local {
            guilds: &mut enabled,
        },
    })?;
    Ok(HttpResponse::Ok().body(rendered))
}
