dirmod::all!(default priv; priv use result);

use std::io;

use actix_web::{guard, middleware, web, HttpResponse};

use crate::index::Index;
use crate::{discord, Secrets};

#[actix_rt::main]
pub async fn run(secrets: Secrets, index: Index, bridge: discord::Bridge) -> io::Result<()> {
    let bridge = web::Data::new(bridge);
    let index = web::Data::new(index);

    let global = web::Data::new(html::GlobalArgs {
        domain: secrets.web().domain().clone(),
        invite_link: discord::invite_link(*secrets.discord().client_id()),
        runtime_id: rand::random(),
    });

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(bridge.clone())
            .app_data(index.clone())
            .app_data(global.clone())
            .wrap(middleware::Logger::default())
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
    .start()
    .await
}
