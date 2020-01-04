dirmod::all!(default priv; priv use result, session, entropy);

use std::io;
use std::time;

use actix_session::CookieSession;
use actix_web::{guard, http, middleware, web, HttpResponse};
use percent_encoding as pe;

use crate::index::Index;
use crate::{discord, Secrets};

#[actix_rt::main]
pub async fn run(secrets: Secrets, index: Index, bridge: discord::Bridge) -> io::Result<()> {
    let entropy_raw = *secrets.web().entropy();

    let secrets_data = web::Data::new(secrets.clone());
    let bridge = web::Data::new(bridge);
    let index = web::Data::new(index);
    let entropy = web::Data::new(Entropy::new(&entropy_raw));
    let common_client = web::Data::new(reqwest::Client::new());

    let global = web::Data::new(html::GlobalArgs {
        domain: secrets.web().domain().clone(),
        runtime_id: rand::random(),
        invite_link: format!(
            "https://discordapp.com/oauth2/authorize?\
            client_id={client_id}&\
            permissions=68608&\
            redirect_uri={domain}%2Fauth&\
            response_type=code&\
            scope=identify%20bot%20guilds",
            client_id = *secrets.discord().client_id(),
            domain = pe::utf8_percent_encode(secrets.web().domain(), pe::NON_ALPHANUMERIC),
        ),
    });

    async fn error404(global: web::Data<html::GlobalArgs>) -> UserResult<HttpResponse> {
        let rendered = html::error::render(
            global.as_ref(),
            html::PageArgs {
                config: (),
                title: "Not Found",
                description: "404 Not Found",
                login: None,
            },
            html::error::Local {
                message:
                    "This route does not exist. Perhaps there would be something here one day?",
            },
        )?;
        Ok(HttpResponse::NotFound().body(rendered))
    }

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(secrets_data.clone())
            .app_data(bridge.clone())
            .app_data(index.clone())
            .app_data(entropy.clone())
            .app_data(common_client.clone())
            .app_data(global.clone())
            .wrap(middleware::Logger::default())
            .wrap(CookieSession::private(&entropy_raw).name("wc"))
            .service(index::index)
            .service(assets::script)
            .service(assets::style)
            .service(auth::invite)
            .service(auth::login)
            .service(auth::logout)
            .service(account::handler)
            .service(guild::handler)
            .service(guilds::handler)
            .default_service(
                web::resource("").route(web::get().to(error404)).route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
            )
    })
    .bind(secrets.web().addr())?
    .run()
    .await
}

type Login = SessData<LoginInfo>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct LoginInfo {
    timeout: time::SystemTime,
    token: String,
    disp: html::UserDisp,
}

impl SessField for LoginInfo {
    fn key() -> &'static str {
        "login"
    }
}

fn redirect<S: AsRef<str>>(path: S) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, path.as_ref())
        .finish()
}
