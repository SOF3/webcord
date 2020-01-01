use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::{html, UserResult};
use crate::{block, discord};

#[actix_web::get("/guilds")]
pub(super) async fn handler(
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    web::Query(page_data): web::Query<PageData>,
) -> UserResult<HttpResponse> {
    let guilds = block(move || bridge.list_guilds())
        .await
        .map_err(global.priv_error("Error querying Discord API"))?; // TODO paginate
    let count = guilds.len();
    let rendered = html::guilds::render(html::Args {
        global: global.as_ref(),
        page: html::PageArgs {
            title: "Guilds mirrored by webcord",
            description: &format!("webcord is mirroring chat from {} guilds", count),
        },
        local: html::guilds::Local {
            guilds: &mut guilds.iter().map(|(id, name)| html::guilds::GuildEntry {
                id: *id,
                name: name.as_str(),
            }),
            current_page: page_data.page,
            total_pages: 1, // TODO fix
        },
    })?;
    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Deserialize)]
pub(super) struct PageData {
    page: usize,
}
