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
        .map_err(global.priv_error("Error loading guild information from Discord"))?
        .json::<Vec<PartialGuild>>()
        .await
        .map_err(global.priv_error("Error loading guild information from Discord"))?;

    let with_admin = guilds
        .iter()
        .filter(|guild| guild.permissions & 8 == 8)
        .map(|guild| Ok((guild.id.parse::<u64>()? as GuildId, guild)))
        .collect::<Result<HashMap<_, _>, std::num::ParseIntError>>()
        .map_err(global.priv_error("Error loading guild information from Discord"))?;

    let enabled = index
        .filter_enabled(with_admin.keys().copied())
        .map_err(global.priv_error("Error loading guild configuration"))?
        .into_iter()
        .map(|guild| {
            let object = &with_admin[&guild.guild_id];
            html::account::GuildEntry {
                id: guild.guild_id,
                name: &object.name,
                icon: object.icon.as_ref().map(|s| s.as_str()),
                listed: guild.listed,
            }
        })
        .collect::<Vec<_>>();

    let rendered = html::account::render(
        global.as_ref(),
        html::PageArgs {
            config: client::PageConfig {
                guilds: serde_iter::CloneOnce::from(enabled.iter().map(|guild| {
                    client::GuildEntry {
                        id: guild.id as u64,
                        listed: guild.listed,
                    }
                })),
            },
            title: "Manage account",
            description: "Manage guilds under your account",
            login: Some(&login.disp),
        },
        html::account::Local {
            guilds: &mut enabled.iter(),
        },
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}

mod client {
    #[derive(serde::Serialize)]
    pub(super) struct PageConfig<I: Iterator<Item = GuildEntry> + Clone> {
        #[serde(with = "serde_iter::seq")]
        pub(super) guilds: I,
    }

    #[derive(serde::Serialize)]
    pub(super) struct GuildEntry {
        #[serde(serialize_with = "u64_to_string")]
        pub(super) id: u64,
        pub(super) listed: bool,
    }

    fn u64_to_string<S: serde::Serializer>(v: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&v.to_string())
    }

    impl<'t, I> crate::web::html::PageConfig for PageConfig<I>
    where
        I: Iterator<Item = GuildEntry> + Clone,
    {
        fn page_type() -> &'static str {
            "account"
        }
    }
}
