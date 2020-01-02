use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

use actix_session::Session;
use actix_web::{dev, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};

pub(super) struct SessData<T: SessField + Serialize> {
    session: Session,
    value: Option<T>,
}

impl<T: Debug + SessField + Serialize> Debug for SessData<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: SessField + Serialize> Deref for SessData<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Option<T> {
        &self.value
    }
}

impl<T: SessField + Serialize> SessData<T> {
    pub(super) fn write(&mut self) -> SessionWriteGuard<'_, T> {
        SessionWriteGuard(self)
    }
}

#[derive(Debug)]
pub(super) struct SessionWriteGuard<'t, T: SessField + Serialize>(&'t mut SessData<T>);

impl<'t, T: SessField + Serialize> Deref for SessionWriteGuard<'t, T> {
    type Target = Option<T>;

    fn deref(&self) -> &Option<T> {
        &self.0.value
    }
}

impl<'t, T: SessField + Serialize> DerefMut for SessionWriteGuard<'t, T> {
    fn deref_mut(&mut self) -> &mut Option<T> {
        &mut self.0.value
    }
}

impl<'t, T: SessField + Serialize> Drop for SessionWriteGuard<'t, T> {
    fn drop(&mut self) {
        if let Some(value) = self.0.value.take() {
            let result = self.0.session.set(<T as SessField>::key(), value);
            if let Err(err) = result {
                log::error!("Failed writing cookie upon SessionWriteGuard drop: {}", err);
            }
        } else {
            self.0.session.remove(<T as SessField>::key());
        }
    }
}

impl<T: SessField + Serialize + for<'de> Deserialize<'de>> FromRequest for SessData<T> {
    type Error = <Session as FromRequest>::Error;
    type Future = futures::future::Map<
        <Session as FromRequest>::Future,
        fn(Result<Session, Self::Error>) -> Result<Self, Self::Error>,
    >;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        use futures::future::FutureExt;

        <Session as FromRequest>::from_request(req, payload).map(|session| {
            let session = session?;
            let value = match session.get::<T>(<T as SessField>::key())? {
                Some(t) => Some(t),
                None => None,
            };
            let data = Self { session, value };
            Ok(data)
        })
    }
}

pub(super) trait SessField {
    fn key() -> &'static str;
}
