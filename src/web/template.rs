use std::io;

use actix_web::error;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

use crate::{discord, Secrets};

pub(super) struct Templates(Handlebars, GlobalArgs);

macro_rules! decl_tmpl {
    ($( $name:ident $(<$($tyn:ident $(: $typ:ty)?),*>)? ($arg:ty); )*) => {
        $(
            pub(super) fn $name(&self, page: &PageArgs<'_ $(, $($tyn:ident $(: $typ:ty)?),*)?>, arg: &$arg) -> Result<String, error::Error> {
                self.0.render(stringify!($name), &json!({
                    "global": self.1,
                    "page": page,
                    "local": arg,
                }))
                .map_err(super::internal_error("Handlebars rendering error"))
            }
        )*
    };
}

impl Templates {
    pub(super) fn try_new(secrets: &Secrets) -> io::Result<Self> {
        let mut hb = Handlebars::new();
        hb.register_templates_directory(".hbs", "./templates")
            .map_err(crate::ctx("loading handlebars templates"))?;
        let ga = GlobalArgs::new(secrets);
        Ok(Self(hb, ga))
    }

    decl_tmpl! {
        index(IndexArgs);
        error404(ErrorArgs<'_>);
        error500(ErrorArgs<'_>);

        guild(GuildArgs<'_>);
    }
}

#[derive(Serialize)]
pub(super) struct GlobalArgs {
    domain: String,
    invite_link: String,
}

impl GlobalArgs {
    pub(super) fn new(secrets: &Secrets) -> Self {
        Self {
            domain: secrets.web().domain().clone(),
            invite_link: discord::invite_link(*secrets.discord().client_id()),
        }
    }
}

#[derive(Serialize)]
pub(super) struct PageArgs<'t> {
    pub(super) title: &'t str,
    pub(super) description: &'t str,
}

#[derive(Serialize)]
pub(super) struct IndexArgs {}

#[derive(Serialize)]
pub(super) struct ErrorArgs<'t> {
    pub(super) message: &'t str,
}

#[derive(Serialize)]
pub(super) struct GuildArgs<'t> {
    pub(super) guild: Guild<'t>,
    pub(super) channels: &'t Vec<(u64, String)>,
}

#[derive(Serialize)]
pub(super) struct Guild<'t> {
    pub(super) id: u64,
    pub(super) name: &'t str,
}
