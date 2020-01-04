use std::convert::TryInto;
use std::fmt;

use actix_web::http::StatusCode;

use super::{html, lib, GlobalArgs, Output, PageArgs, PageConfig};
use crate::web::UserError;

pub fn render<'t, C: PageConfig>(
    global: &'t GlobalArgs,
    page: PageArgs<'t, C>,
    local: Local<'t>,
) -> Output {
    let title = &page.title;
    lib::minimal_layout(
        global,
        &page,
        html! {
            div(class = "container section") {
                h3: title;
                p(class = "light"): local.message;
            }
        },
    )
}

#[derive(Debug, Clone)]
pub struct Local<'t> {
    pub message: &'t str,
}

impl GlobalArgs {
    pub(in crate::web) fn user_error<C>(&self, code: C, message: impl fmt::Display) -> UserError
    where
        C: TryInto<StatusCode>,
        C::Error: fmt::Debug,
    {
        let message = message.to_string();
        let code = code.try_into().expect("Invalid status code");
        let body = super::error::render(
            &self,
            PageArgs {
                config: (),
                title: &format!(
                    "{} {}",
                    code.as_str(),
                    code.canonical_reason().unwrap_or("")
                ),
                description: &message,
                login: None,
            },
            super::error::Local { message: &message },
        );
        match body {
            Ok(body) => UserError {
                code,
                inner: message.into(),
                body: body.into(),
            },
            Err(critical) => critical.into(),
        }
    }

    pub(in crate::web) fn priv_error<'a, T, R>(
        &'a self,
        readable: &'a R,
    ) -> impl (FnOnce(T) -> UserError) + 'a
    where
        T: fmt::Display,
        R: fmt::Display + ?Sized,
    {
        move |err| {
            log::error!("Internal error: {}", err);
            self.user_error(500, readable)
        }
    }

    pub(in crate::web) fn priv_error_code<'a, C, T, R>(
        &'a self,
        code: C,
        readable: &'a R,
    ) -> impl (FnOnce(T) -> UserError) + 'a
    where
        T: fmt::Display,
        R: fmt::Display + ?Sized,
        C: TryInto<StatusCode> + 'a,
        C::Error: fmt::Debug,
    {
        move |err| {
            log::error!("Internal error: {}", err);
            self.user_error(code, readable)
        }
    }
}
