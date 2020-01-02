use actix_web::{http, web, HttpResponse};
use serde::{Deserialize, Serialize};

use super::{html, UserResult};
use crate::Secrets;

#[actix_web::get("/invite")]
pub(super) async fn invite(global: web::Data<html::GlobalArgs>) -> HttpResponse {
    HttpResponse::PermanentRedirect()
        .header(http::header::LOCATION, global.invite_link.as_str())
        .finish()
}

#[actix_web::get("/auth")]
pub(super) async fn login(
    mut session: super::Login,
    args: web::Query<LoginArgs>,
    secrets: web::Data<Secrets>,
    global: web::Data<html::GlobalArgs>,
    common: web::Data<reqwest::Client>,
) -> UserResult<HttpResponse> {
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
            scope: "identify bot",
        })
        .send()
        .await
        .map_err(global.priv_error("Error identifying user"))?
        .json::<TokenResponse>()
        .await
        .map_err(global.priv_error("Error identifying user"))?;

    if !resp.scope.split(" ").any(|scope| scope == "identify") {
        return Err(global.user_error(401, "Requested identify scope, did not get identify"));
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
            // token: resp.access_token,
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

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/account")
        .finish())
}

#[derive(Deserialize)]
pub(super) struct LoginArgs {
    code: String,
}

#[actix_web::get("/logout")]
pub(super) async fn logout(mut session: super::Login) -> UserResult<HttpResponse> {
    {
        let mut write = session.write();
        *write = None;
    }
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish())
}
