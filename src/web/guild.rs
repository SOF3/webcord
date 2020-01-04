use actix_web::{web, HttpResponse};

use super::{html, UserResult};
use crate::{block, discord, GuildId};

#[actix_web::get("/guilds/{guild}")]
pub(super) async fn handler(
    bridge: web::Data<discord::Bridge>,
    global: web::Data<html::GlobalArgs>,
    path: web::Path<(u64,)>,
) -> UserResult<HttpResponse> {
    let (guild_id,) = path.into_inner();
    let guild_id = guild_id as GuildId;
    let guild = block(move || bridge.guild_info(guild_id, false))
        .await
        .map_err(global.priv_error_code(404, "Guild does not exist"))?;
    if guild.channels().len() > 0 {
        Ok(super::redirect(format!(
            "/guilds/{}/{}",
            guild_id,
            guild.channels()[0].name()
        )))
    } else {
        unimplemented!("Display error page about this guild having no visible channels")
    }
}
