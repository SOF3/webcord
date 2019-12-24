use super::{html, lib, text, Args};
use crate::GuildId;

pub fn render<'t, I>(args: Args<'t, Local<'t, I>>) -> String
where
    I: Iterator<Item = GuildEntry<'t>>,
{
    lib::layout(args, |args| {
        html! {<main>
             <div class="container section">
                <div class="row">
                    <div class="col s12 m6">"Guilds mirrored by webcord"</div>
                    {{ pages(args.local.current_page, args.local.total_pages) }}
                </div>
                <div class="row">
                    { args.local.guilds.map(|guild| html! {
                        <div class="truncate col s12 m3">
                            <a href={ &format!("/guilds/{}", guild.id) }>
                                { text!("{}", guild.name) }
                            </a>
                        </div>
                    }) }
                </div>
            </div>
        </main>}
    })
}

fn pages(current: usize, n: usize) -> super::Dom {
    html! {
        <div class="col s12 m6"> "Page "
            <select name="guilds-select-page" id="guilds-select-page">
                { (1..=n).map(|i| {
                    html! {
                        <option value={ i.to_string() } selected={ i == current}>
                            { text!("{}", i) }
                        </option>
                    }
                }) }
            </select>
            { text!(" of {}", n) }
        </div>
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
