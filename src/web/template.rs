use std::convert::TryInto;
use std::fmt;
use std::io;
use std::sync::Arc;

use actix_web::http::StatusCode;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

use super::*;
use crate::{discord, Secrets};

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

    pub(super) fn user_error<C>(self: Arc<Self>, code: C, inner: impl fmt::Display) -> UserError
    where
        C: TryInto<StatusCode>,
        C::Error: fmt::Debug,
    {
        user_error(self, code, inner)
    }

    pub(super) fn priv_error<T>(
        self: Arc<Self>,
        readable: impl fmt::Display,
    ) -> impl FnOnce(T) -> UserError
    where
        T: fmt::Display,
    {
        |err| {
            log::error!("Internal error: {}", err);
            self.user_error(500, readable)
        }
    }
}

macro_rules! decl_tmpl {
    ($( $name:ident $(<$($tyn:ident $(: $typ:ty)?),*>)? ($arg:ty); )*) => {
        impl Templates {
            $(
                pub(super) fn $name(self: Arc<Self>, page: &PageArgs<'_ $(, $($tyn:ident $(: $typ:ty)?),*)?>, arg: &$arg) -> UserResult<String> {
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
    pub(super) channels: &'t Vec<(u64, String)>,
}

#[derive(Debug, Serialize)]
pub(super) struct Guild<'t> {
    pub(super) id: u64,
    pub(super) name: &'t str,
}
