use actix_web::{web, HttpResponse};

use super::{html, UserResult};

#[actix_web::get("/")]
pub(super) async fn index(
    login: super::Login,
    global: web::Data<html::GlobalArgs>,
) -> UserResult<HttpResponse> {
    let rendered = html::index::render(
        global.as_ref(),
        html::PageArgs {
            config: client::PageConfig {},
            title: "webcord",
            description: "webcord: Chat log mirror for Discord",
            login: login.as_ref().map(|login| &login.disp),
        },
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}

mod client {
    #[derive(serde::Serialize)]
    pub(super) struct PageConfig {}

    impl crate::web::html::PageConfig for PageConfig {
        fn page_type() -> &'static str {
            "index"
        }
    }
}
