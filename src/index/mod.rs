dirmod::all!(default pub use; default dir pub);

use derive_more::{Display, From};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use crate::Secrets;

type ConnMan = ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ConnMan>;

#[derive(Clone)]
pub struct Index(Pool);

impl Index {
    pub fn try_new(secrets: &Secrets) -> Result<Index, r2d2::Error> {
        Ok(Index(Pool::new(ConnMan::new(secrets.database().url()))?))
    }
}

#[derive(From, Display)]
pub enum QueryError {
    R2d2(r2d2::Error),
    Diesel(diesel::result::Error),
}
