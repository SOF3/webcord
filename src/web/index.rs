use actix_web::{web, HttpResponse, Result};

use super::template::{self, Templates};

#[actix_web::get("/")]
pub(super) async fn index(tmpl: web::Data<Templates>) -> Result<HttpResponse> {
    let rendered = tmpl.index(
        &template::PageArgs {
            title: "webcord",
            description: "webcord: Chat log mirror for Discord",
        },
        &template::IndexArgs {},
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub(super) async fn error404(tmpl: web::Data<Templates>) -> Result<HttpResponse> {
    let rendered = tmpl.error404(
        &template::PageArgs {
            title: "Not Found",
            description: "404 Not Found",
        },
        &template::ErrorArgs {
            message: "This route does not exist",
        },
    )?;
    Ok(HttpResponse::NotFound().body(rendered))
}
