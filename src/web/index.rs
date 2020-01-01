use actix_web::{web, HttpResponse};

use super::{html, UserResult};

#[actix_web::get("/")]
pub(super) async fn index(
    login: super::Login,
    global: web::Data<html::GlobalArgs>,
) -> UserResult<HttpResponse> {
    let rendered = html::index::render(html::Args {
        global: global.as_ref(),
        page: html::PageArgs {
            title: "webcord",
            description: "webcord: Chat log mirror for Discord",
            login: login.as_ref().map(|login| &login.disp),
        },
        local: (),
    })?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub(super) async fn error404(global: web::Data<html::GlobalArgs>) -> UserResult<HttpResponse> {
    let rendered = html::error::render(html::Args {
        global: global.as_ref(),
        page: html::PageArgs {
            title: "Not Found",
            description: "404 Not Found",
            login: None,
        },
        local: html::error::Local {
            message: "This route does not exist. Perhaps there would be something here one day?",
        },
    })?;
    Ok(HttpResponse::NotFound().body(rendered))
}
