dirmod::all!();

use std::io;

use actix_files::NamedFile;
use actix_web::{error, guard, web, HttpResponse, Result};
use handlebars::Handlebars;
use serde_json::json;

use crate::{discord, Secrets};

#[actix_rt::main]
pub(crate) async fn run(secrets: Secrets, bridge: discord::Bridge) -> io::Result<()> {
    let bridge = web::Data::new(bridge);

    let mut hb = handlebars::Handlebars::new();
    hb.register_templates_directory(".hbs", "./static/templates")
        .map_err(crate::ctx("loading handlebars templates"))?;
    let hb = web::Data::new(hb);

    let gha = json!({
        "domain": secrets.web().domain(),
        "invite_link": discord::invite_link(*secrets.discord().client_id()),
    });
    let gha = web::Data::new(GlobalHbArgs(gha));

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .register_data(bridge.clone())
            .register_data(hb.clone())
            .register_data(gha.clone())
            .service(index)
            .service(script)
            .service(guild_page)
            .service(
                web::resource("").route(web::get().to(error404)).route(
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

#[actix_web::get("/")]
async fn index(hb: web::Data<Handlebars>, gha: web::Data<GlobalHbArgs>) -> Result<HttpResponse> {
    let data = json!({
        "global": &gha.0,
    });
    let rendered = hb
        .render("index", &data)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

#[actix_web::get("/script.js")]
async fn script() -> Result<NamedFile> {
    Ok(NamedFile::open("build/script.js")?)
}

async fn error404(hb: web::Data<Handlebars>, gha: web::Data<GlobalHbArgs>) -> Result<HttpResponse> {
    let data = json!({
        "global": &gha.0,
    });
    let rendered = hb
        .render("404", &data)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

struct GlobalHbArgs(pub serde_json::Value);
