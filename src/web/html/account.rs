use super::{html, lib, Args, Output};
use crate::model::GuildId;

pub fn render<'t, I>(
    Args {
        global,
        page,
        local,
    }: Args<'t, Local<'t, I>>,
) -> Output
where
    I: Iterator<Item = GuildEntry<'t>>,
{
    lib::layout(
        global,
        page,
        html! {
            main {
                div(class = "container section") {
                    div(class = "collection") {
                        @ for guild in local.guilds {
                            a(href = format_args!("/account/{}", guild.id), class = "collection-item avatar") {
                                @ if let Some(icon) = guild.icon {
                                    img(class = "circle", src = format_args!("https://cdn.discordapp.com/icons/{}/{}", guild.id as u64, icon));
                                }
                                span(class = "title"): guild.name;
                            }
                        }
                    }
                }
            }
        },
    )
}

pub struct Local<'t, I>
where
    I: Iterator<Item = GuildEntry<'t>>,
{
    pub guilds: &'t mut I,
}

pub struct GuildEntry<'t> {
    pub id: GuildId,
    pub name: &'t str,
    pub icon: Option<&'t str>,
}
