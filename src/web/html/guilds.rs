use super::{html, lib, Args, Output};
use crate::GuildId;
use horrorshow::Render;

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
                    div(class = "row") {
                        div(class = "col s12 m6"): "Guilds mirrored by webcord";
                        : pages(local.current_page, local.total_pages);
                    }
                    div(class = "row") {
                        @ for guild in local.guilds {
                            div(class = "truncate col s12 m3") {
                                a(href = format_args!("/guilds/{}", guild.id)): guild.name;
                            }
                        }
                    }
                }
            }
        },
    )
}

fn pages(current: usize, n: usize) -> impl Render {
    html! {
        div(class = "col s12 m6 input-field") {
            select(name = "guilds-select-page", id = "guilds-select-page") {
                @ for i in 1..=n {
                    option(value = format_args!("{}", i), selected ?= (i == current)):
                        format_args!("Page {} of {}", i, n);
                }
            }
            label: "Select page:";
        }
    }
}

#[derive(Debug)]
pub struct Local<'t, I>
where
    I: Iterator<Item = GuildEntry<'t>>,
{
    pub guilds: &'t mut I,
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Debug)]
pub struct GuildEntry<'t> {
    pub id: GuildId,
    pub name: &'t str,
}
