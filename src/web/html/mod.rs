use std::convert::TryInto;
use std::fmt;

use actix_web::http::StatusCode;
use typed_html::{html, text};

use super::UserError;

dirmod::all!(default pub(super); priv use lib);

type Dom = Box<dyn typed_html::elements::FlowContent<String>>;

#[derive(Debug)]
pub struct Args<'t, T> {
    pub global: &'t GlobalArgs,
    pub page: PageArgs<'t>,
    pub local: T,
}

#[derive(Debug)]
pub struct GlobalArgs {
    pub domain: String,
    pub invite_link: String,
    pub runtime_id: u64,
}

impl GlobalArgs {
    pub(super) fn user_error<C>(&self, code: C, inner: impl fmt::Display) -> UserError
    where
        C: TryInto<StatusCode>,
        C::Error: fmt::Debug,
    {
        let message = inner.to_string();
        let code = code.try_into().expect("Invalid status code");
        let body = error::render(Args {
            global: &self,
            page: PageArgs {
                title: &format!(
                    "{} {}",
                    code.as_str(),
                    code.canonical_reason().unwrap_or("")
                ),
                description: &message,
            },
            local: error::Local { message: &message },
        });
        UserError {
            code,
            inner: message.into(),
            body: body.into(),
        }
    }

    pub(super) fn priv_error<'a, T, R>(
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

#[derive(Debug)]
pub struct PageArgs<'t> {
    pub title: &'t str,
    pub description: &'t str,
}
