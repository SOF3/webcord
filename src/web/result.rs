use std::borrow::Cow;
use std::fmt;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub(super) struct UserError {
    pub(super) code: StatusCode,
    pub(super) inner: Cow<'static, str>,
    pub(super) body: Cow<'static, str>,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.inner.as_ref())
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        if self.inner.as_ref().len() > 0 {
            log::warn!("User error: {}", self.inner.as_ref());
        }

        let mut builder = HttpResponse::build(self.code);
        match &self.body {
            Cow::Borrowed(body) => builder.body(*body),
            Cow::Owned(body) => builder.body(body),
        }
    }
}

pub(super) type UserResult<T = (), E = UserError> = std::result::Result<T, E>;

pub(super) struct Critical;

impl From<Critical> for UserError {
    fn from(_: Critical) -> Self {
        UserError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            inner: Cow::Borrowed(""),
            body: Cow::Borrowed("A critical internal error happened. We can't even display a beautiful error page :("),
        }
    }
}
