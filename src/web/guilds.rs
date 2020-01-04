use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::{html, UserResult};
use crate::{block, discord};

#[actix_web::get("/guilds")]
pub(super) async fn handler(
    login: super::Login,
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    web::Query(page_data): web::Query<PageData>,
) -> UserResult<HttpResponse> {
    let guilds = block(move || bridge.list_guilds())
        .await
        .map_err(global.priv_error("Error querying Discord API"))?; // TODO paginate
    dbg!(&guilds);
    let count = guilds.len();
    let rendered = html::guilds::render(
        global.as_ref(),
        html::PageArgs {
            config: client::PageConfig {},
            title: "Guilds mirrored by webcord",
            description: &format!("webcord is mirroring chat from {} guilds", count),
            login: login.as_ref().map(|login| &login.disp),
        },
        html::guilds::Local {
            guilds: &mut guilds.iter().map(|(id, name)| html::guilds::GuildEntry {
                id: *id,
                name: name.as_str(),
            }),
            current_page: page_data.page,
            total_pages: 1,
        },
    )?;
    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Deserialize)]
pub(super) struct PageData {
    #[serde(default = "default_one")]
    page: usize,
}

mod client {
    #[derive(serde::Serialize)]
    pub(super) struct PageConfig {}

    impl crate::web::html::PageConfig for PageConfig {
        fn page_type() -> &'static str {
            "guilds"
        }
    }
}

fn default_one() -> usize {
    1
}
