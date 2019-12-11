use actix_web::{error, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

#[actix_web::get("/<guild>")]
pub(super) async fn guild_page(
    hb: web::Data<Handlebars>,
    gha: web::Data<super::GlobalHbArgs>,
    path: web::Path<(u64,)>,
) -> Result<HttpResponse, error::Error> {
    let (guild,) = path.into_inner();
    let channels = get_guild_channels(guild).await?;
    let data = json!({
        "global": &gha.0,
        "guild": {
            "id": guild,
            // TODO name
        },
        "channels": channels,
    });
    let rendered = hb
        .render("guild", &data)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

async fn get_guild_channels(_guild: u64) -> Result<Vec<(u64, String)>, error::Error> {
    unimplemented!()
}
