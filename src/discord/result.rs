#[derive(derive_more::From, derive_more::Display)]
pub enum Error {
    Serenity(serenity::Error),
    Index(crate::index::QueryError),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
