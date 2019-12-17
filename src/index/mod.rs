use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use crate::Secrets;

type ConnMan = ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ConnMan>;

#[derive(Clone)]
pub(crate) struct Index(Pool);

impl Index {
    pub(crate) fn try_new(secrets: &Secrets) -> Result<Index, r2d2::Error> {
        Ok(Index(Pool::new(ConnMan::new(secrets.database().url()))?))
    }
}
