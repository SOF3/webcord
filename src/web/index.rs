use actix_web::{web, HttpResponse};

use super::template::{self, Templates};
use super::UserResult;

#[actix_web::get("/")]
pub(super) async fn index(tmpl: web::Data<Templates>) -> UserResult<HttpResponse> {
    let rendered = tmpl.into_inner().index(
        &template::PageArgs {
            title: "webcord",
            description: "webcord: Chat log mirror for Discord",
        },
        &template::IndexArgs {},
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub(super) async fn error404(tmpl: web::Data<Templates>) -> actix_web::Result<HttpResponse> {
    let rendered = tmpl
        .into_inner()
        .error(
            &template::PageArgs {
                title: "Not Found",
                description: "404 Not Found",
            },
            &template::ErrorArgs {
                message:
                    "This route does not exist. Perhaps there would be something here one day?",
            },
        )
        .map_err(super::internal_error("Template rendering error"))?;
    Ok(HttpResponse::NotFound().body(rendered))
}
