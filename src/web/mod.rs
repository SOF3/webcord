dirmod::all!(default priv; priv use result);

use std::io;

use actix_session::{CookieSession, Session};
use actix_web::{cookie, dev, guard, middleware, web, FromRequest, HttpRequest, HttpResponse};
use derive_more::{Deref, From};

use crate::index::Index;
use crate::{discord, Secrets};

#[actix_rt::main]
pub async fn run(secrets: Secrets, index: Index, bridge: discord::Bridge) -> io::Result<()> {
    let bridge = web::Data::new(bridge);
    let index = web::Data::new(index);

    let global = web::Data::new(html::GlobalArgs {
        domain: secrets.web().domain().clone(),
        runtime_id: rand::random(),
    });

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(bridge.clone())
            .app_data(index.clone())
            .app_data(global.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                CookieSession::private(&rand::random::<[u8; 32]>())
                    .name("wc")
                    .same_site(cookie::SameSite::Strict),
            )
            .service(index::index)
            .service(assets::script)
            .service(assets::style)
            .service(guild::handler)
            .service(guilds::handler)
            .default_service(
                web::resource("")
                    .route(web::get().to(index::error404))
                    .route(
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

#[derive(Debug, Deref, From, Default)]
pub struct Login(Option<LoginInfo>);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginInfo {
    token: String,
    disp: html::UserDisp,
}

impl FromRequest for Login {
    type Error = <Session as FromRequest>::Error;
    type Future = futures::future::Map<
        <Session as FromRequest>::Future,
        fn(Result<Session, Self::Error>) -> Result<Self, Self::Error>,
    >;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        use futures::future::FutureExt;

        <Session as FromRequest>::from_request(req, payload).map(|session| {
            let session = session?;
            let login = match session.get::<LoginInfo>("login")? {
                Some(info) => Some(info).into(),
                None => Self::default(),
            };
            Ok(login)
        })
    }
}
