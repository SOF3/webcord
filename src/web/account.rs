use actix_web::{web, HttpResponse};

use super::{html, UserResult};

#[actix_web::get("/account")]
pub(super) async fn handler(
    login: super::Login,
    global: web::Data<html::GlobalArgs>,
    common_client: web::Data<reqwest::Client>,
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

    let mut useful = guilds
        .iter()
        .filter(|guild| guild.permissions & 8 == 8)
        .map(|guild| html::account::GuildEntry {
            id: guild.id.parse::<u64>().unwrap_or(0) as crate::model::GuildId, // ignored error
            name: &guild.name,
            icon: guild.icon.as_ref().map(|s| s.as_str()),
        });
    // TODO filter out guilds without bot

    let rendered = html::account::render(html::Args {
        global: global.as_ref(),
        page: html::PageArgs {
            title: "Manage account",
            description: "Manage guilds under your account",
            login: Some(&login.disp),
        },
        local: html::account::Local {
            guilds: &mut useful,
        },
    })?;
    Ok(HttpResponse::Ok().body(rendered))
}
