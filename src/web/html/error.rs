use std::convert::TryInto;
use std::fmt;

use actix_web::http::StatusCode;

use super::{html, lib, Args, GlobalArgs, Output, PageArgs};
use crate::web::UserError;

pub fn render<'t>(
    Args {
        global,
        page,
        local,
    }: Args<'t, Local<'t>>,
) -> Output {
    lib::minimal_layout(
        global,
        page,
        html! {
            main {
                div(class = "container section") {
                    h3: page.title;
                    p(class = "light"): local.message;
                }
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
        let body = super::error::render(Args {
            global: &self,
            page: PageArgs {
                title: &format!(
                    "{} {}",
                    code.as_str(),
                    code.canonical_reason().unwrap_or("")
                ),
                description: &message,
                login: None,
            },
            local: super::error::Local { message: &message },
        });
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
}
