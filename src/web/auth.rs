use std::time;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use super::{html, UserResult};
use crate::Secrets;

const REQUIRED_SCOPES: [&'static str; 3] = ["identify", "bot", "guilds"];

#[actix_web::get("/invite")]
pub(super) async fn invite(global: web::Data<html::GlobalArgs>) -> HttpResponse {
    super::redirect(&global.invite_link)
}

#[actix_web::get("/auth")]
pub(super) async fn login(
    mut session: super::Login,
    args: web::Query<LoginArgs>,
    secrets: web::Data<Secrets>,
    global: web::Data<html::GlobalArgs>,
    common: web::Data<reqwest::Client>,
) -> UserResult<HttpResponse> {
    use itertools::Itertools;

    #[derive(Serialize)]
    struct TokenForm<'t> {
        client_id: u64,
        client_secret: &'t str,
        grant_type: &'t str,
        code: &'t str,
        redirect_uri: &'t str,
        scope: &'t str,
    }
    #[derive(Deserialize)]
    struct TokenResponse {
        access_token: String,
        // token_type: String,
        // expires_in: u64,
        // refresh_token: String,
        scope: String,
    }

    let resp = common
        .post("https://discordapp.com/api/oauth2/token")
        .form(&TokenForm {
            client_id: *secrets.discord().client_id(),
            client_secret: secrets.discord().client_secret(),
            grant_type: "authorization_code",
            code: &args.code,
            redirect_uri: &format!("{}{}", secrets.web().domain(), "/auth"),
            scope: &REQUIRED_SCOPES.iter().join(" "),
        })
        .send()
        .await
        .map_err(global.priv_error("Error identifying user"))?
        .json::<TokenResponse>()
        .await
        .map_err(global.priv_error("Error identifying user"))?;

    for required in &REQUIRED_SCOPES {
        if !resp.scope.split(" ").any(|scope| &scope == required) {
            use itertools::Itertools;
            return Err(global.user_error(
                401,
                format!("Required scopes: {}", REQUIRED_SCOPES.iter().join(" ")),
            ));
        }
    }

    #[derive(Deserialize)]
    struct CurrentUserResponse {
        id: String,
        username: String,
        discriminator: String,
        avatar: Option<String>,
    }
    let user = common
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(&resp.access_token)
        .send()
        .await
        .map_err(global.priv_error("Error identifying user"))?
        .json::<CurrentUserResponse>()
        .await
        .map_err(global.priv_error("Error identifying user"))?;

    {
        let mut write = session.write();
        *write = Some(super::LoginInfo {
            timeout: time::SystemTime::now() + time::Duration::from_secs(3600),
            token: resp.access_token,
            disp: super::html::UserDisp {
                id: user
                    .id
                    .parse()
                    .map_err(global.priv_error("Error identifying user"))?,
                username: user.username.clone(),
                discrim: user.discriminator.clone(),
                avatar: user.avatar,
            },
        });
    }

    Ok(super::redirect(format!("/account#guild-{}", args.guild_id)))
}

#[derive(Deserialize)]
pub(super) struct LoginArgs {
    code: String,
    guild_id: u64,
}

#[actix_web::get("/logout")]
pub(super) async fn logout(mut session: super::Login) -> UserResult<HttpResponse> {
    {
        let mut write = session.write();
        *write = None;
    }
    Ok(super::redirect("/"))
}
