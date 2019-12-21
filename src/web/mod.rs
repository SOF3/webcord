dirmod::all!(default priv; priv use result);

use std::fmt;
use std::io;

use actix_web::{error, guard, middleware, web, HttpResponse};

use crate::index::Index;
use crate::{discord, Secrets};

#[actix_rt::main]
pub async fn run(secrets: Secrets, index: Index, bridge: discord::Bridge) -> io::Result<()> {
    let bridge = web::Data::new(bridge);
    let index = web::Data::new(index);

    let tmpl = template::Templates::try_new(&secrets)?;
    let tmpl = web::Data::new(tmpl);

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(bridge.clone())
            .app_data(index.clone())
            .app_data(tmpl.clone())
            .wrap(middleware::Logger::default())
            .service(index::index)
            .service(assets::script)
            .service(assets::style)
            .service(guild::handler)
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

fn internal_error<D: fmt::Debug + fmt::Display + 'static, E: fmt::Display>(
    user_msg: D,
) -> impl FnOnce(E) -> error::Error {
    |err| {
        log::error!("Error handling request: {}", err);
        error::ErrorInternalServerError(user_msg)
    }
}
