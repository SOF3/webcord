use super::{html, lib, Args, Output};
use crate::{ChannelId, GuildId};

pub fn render<'t>(
    Args {
        global,
        page,
        local: _,
    }: Args<'t, Local<'t>>,
) -> Output {
    lib::layout(
        global,
        page,
        html! {
            div(class = "container section") {
                : "This will specify information about a guild"
            }
        },
    )
}

#[derive(Debug)]
pub struct Local<'t> {
    pub guild: Guild<'t>,
    pub channels: &'t Vec<(ChannelId, &'t str)>,
}

#[derive(Debug)]
pub struct Guild<'t> {
    pub id: GuildId,
    pub name: &'t str,
}
