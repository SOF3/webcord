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
            div(class = "container section") {
                ul(class = "collection") {
                    @ for guild in local.guilds {
                        li(class = "collection-item avatar") {
                            @ if let Some(icon) = guild.icon {
                                img(class = "responsive-img circle", src = format_args!("https://cdn.discordapp.com/icons/{}/{}", guild.id as u64, icon));
                            }
                            span(class = "title") {
                                a(name = format_args!("guild-{}", guild.id), href = format_args!("#guild-{}", guild.id)): guild.name;
                            }
                            p {
                                label {
                                    input(type = "checkbox", class = "filled-in", checked ?= guild.listed);
                                    span: "Listed";
                                }
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
    pub listed: bool,
}
