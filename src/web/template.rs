use std::convert::TryInto;
use std::fmt;
use std::io;

use actix_web::http::StatusCode;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

use super::*;
use crate::{discord, ChannelId, GuildId, Secrets};

#[derive(Debug)]
pub(super) struct Templates(Handlebars, GlobalArgs);

impl Templates {
    pub(super) fn try_new(secrets: &Secrets) -> io::Result<Self> {
        let mut hb = Handlebars::new();
        hb.register_templates_directory(".hbs", "./templates")
            .map_err(crate::ctx("loading handlebars templates"))?;
        let ga = GlobalArgs::new(secrets);
        Ok(Self(hb, ga))
    }

    pub(super) fn user_error<C>(&self, code: C, inner: impl fmt::Display) -> UserError
    where
        C: TryInto<StatusCode>,
        C::Error: fmt::Debug,
    {
        let message = inner.to_string();
        let code = code.try_into().expect("Invalid status code");
        let body = match self.error(
            &PageArgs {
                title: &format!(
                    "{} {}",
                    code.as_str(),
                    code.canonical_reason().unwrap_or("")
                ),
                description: &message,
            },
            &ErrorArgs { message: &message },
        ) {
            Ok(body) => body,
            Err(err) => {
                log::error!("Error printing error template: {}", err);
                return UserError {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    inner: "Error printing error template".into(),
                    body: "Internal server error".into(),
                };
            }
        };
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

macro_rules! decl_tmpl {
    ($( $name:ident $(<$($tyn:ident $(: $typ:ty)?),*>)? ($arg:ty); )*) => {
        impl Templates {
            $(
                pub(super) fn $name $( < $($tyn:ident $(: $typ:ty)?),* > )? (&self, page: &PageArgs<'_>, arg: &$arg) -> UserResult<String> {
                    self.0.render(stringify!($name), &json!({
                        "global": self.1,
                        "page": page,
                        "local": arg,
                    }))
                    .map_err(self.priv_error("Handlebars rendering error"))
                }
             )*
        }
    };
}

decl_tmpl! {
    index(IndexArgs);
    error(ErrorArgs<'_>);

    guild(GuildArgs<'_>);
}

#[derive(Debug, Serialize)]
pub(super) struct GlobalArgs {
    domain: String,
    invite_link: String,
    runtime_id: u64,
}

impl GlobalArgs {
    pub(super) fn new(secrets: &Secrets) -> Self {
        Self {
            domain: secrets.web().domain().clone(),
            invite_link: discord::invite_link(*secrets.discord().client_id()),
            runtime_id: rand::random(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(super) struct PageArgs<'t> {
    pub(super) title: &'t str,
    pub(super) description: &'t str,
}

#[derive(Debug, Serialize)]
pub(super) struct IndexArgs {}

#[derive(Debug, Serialize)]
pub(super) struct ErrorArgs<'t> {
    pub(super) message: &'t str,
}

#[derive(Debug, Serialize)]
pub(super) struct GuildArgs<'t> {
    pub(super) guild: Guild<'t>,
    pub(super) channels: Vec<(ChannelId, &'t str)>,
}

#[derive(Debug, Serialize)]
pub(super) struct Guild<'t> {
    pub(super) id: GuildId,
    pub(super) name: &'t str,
}
