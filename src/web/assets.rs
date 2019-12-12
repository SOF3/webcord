use actix_files::NamedFile;
use actix_web::Result;

#[actix_web::get("/script.js")]
pub(super) async fn script() -> Result<NamedFile> {
    Ok(NamedFile::open("build/script.js")?)
}

#[actix_web::get("/style.css")]
pub(super) async fn style() -> Result<NamedFile> {
    Ok(NamedFile::open("static/style.css")?)
}
