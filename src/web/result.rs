use std::borrow::Cow;
use std::fmt;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub(super) struct UserError {
    code: StatusCode,
    inner: Cow<'static, str>,
    body: Cow<'static, str>,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.inner.as_ref())
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        log::warn!("User error: {}", self.inner.as_ref());

        let builder = HttpResponse::build(self.code);
        match &self.body {
            Cow::Borrowed(body) => builder.body(*body),
            Cow::Owned(body) => builder.body(body),
        }
    }
}

pub(super) type UserResult<T = (), E = UserError> = std::result::Result<T, E>;
