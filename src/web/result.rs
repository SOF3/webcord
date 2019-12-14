use std::convert::TryInto;
use std::fmt;
use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use super::template;

#[derive(Debug)]
pub(super) struct UserError {
    templates: Arc<template::Templates>,
    code: StatusCode,
    inner: String,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", &self.inner)
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let message = self.inner.to_string();

        log::warn!("User error: {}", &message);

        let body = match self.templates.clone().error(
            &template::PageArgs {
                title: &format!(
                    "{} {}",
                    self.code.as_str(),
                    self.code.canonical_reason().unwrap_or("")
                ),
                description: &message,
            },
            &template::ErrorArgs { message: &message },
        ) {
            Ok(body) => body,
            Err(err) => {
                log::error!("Error during printing error template: {}", err);
                return HttpResponse::InternalServerError().body("Internal Server Error");
            }
        };

        HttpResponse::build(self.code).body(body)
    }
}

pub(super) fn user_error<C>(
    templates: Arc<template::Templates>,
    code: C,
    inner: impl fmt::Display,
) -> UserError
where
    C: TryInto<StatusCode>,
    C::Error: fmt::Debug,
{
    UserError {
        templates,
        code: code.try_into().expect("Invalid status code"),
        inner: inner.to_string(),
    }
}

pub(super) type UserResult<T = (), E = UserError> = std::result::Result<T, E>;
