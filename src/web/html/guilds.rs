use super::{html, lib, Args, Output, RenderOnce};
use crate::GuildId;

pub fn render<'t, I>(args: Args<'t, Local<'t, I>>) -> Output
where
    I: Iterator<Item = GuildEntry<'t>>,
{
    lib::layout(args, |global, page, local| {
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
        }
    })
}

fn pages(current: usize, n: usize) -> impl RenderOnce {
    html! {
        div(class = "col s12 m6") {
            : "Page ";
            select(name = "guilds-select-page", id = "guilds-select-page") {
                @ for i in 1..=n {
                    option(value = format_args!("{}", i), selected ?= (i == current)): i;
                }
            }
            : format_args!(" of {}", n);
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
