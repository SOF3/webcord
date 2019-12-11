use std::io;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::Result;

use crate::{discord, Secrets};

#[actix_rt::main]
pub(crate) async fn run(secrets: Secrets, bridge: discord::Bridge) -> io::Result<()> {
    let bridge = Arc::new(bridge);
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(Arc::clone(&bridge))
            .service(index)
    })
    .bind(secrets.web().addr())?
    .start().await
}

#[actix_web::get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}
