// #[derive(derive_more::From)]
// pub enum Error {
// Serenity(serenity::Error),
// }

pub type Result<T = (), E = serenity::Error> = std::result::Result<T, E>;
