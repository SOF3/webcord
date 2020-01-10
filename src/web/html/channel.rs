use std::borrow::Cow;

use super::{html, lib, GlobalArgs, Output, PageArgs, PageConfig, RenderOnce};
use crate::{CategoryId, ChannelId, EmojiId, GuildId, UserId};

pub fn render<'t, Cat, ICatChan>(
    global: &'t GlobalArgs,
    page: PageArgs<'t, impl PageConfig>,
    guild: Guild<'t>,
    groups: Cat,
    current_group: &'t Group<'t, ICatChan>,
    current_channel: Channel<'t>,
    date: chrono::NaiveDate,
    messages: impl Iterator<
        Item = Message<'t, impl RenderOnce + 't, impl Iterator<Item = (Emoji<'t>, u64)>>,
    >,
) -> Output
where
    Cat: IntoIterator<Item = &'t Group<'t, ICatChan>> + Copy,
    ICatChan: IntoIterator<Item = &'t Channel<'t>> + Copy,
{
    use chrono::Datelike;

    let current_channel_id = current_channel.id;
    let guild_id = guild.id;
    lib::full_layout(
        global,
        &page,
        html! {
            link(rel = "canonical", href = format_args!("{domain}/guilds/{guild}/{channel}/{year}/{month}/{day}",
                domain = &global.domain,
                guild = guild_id,
                channel = current_channel_id,
                year = date.year(),
                month = date.month(),
                day = date.day(),
            ));
        },
        html! {
            div(class = "nav-wrapper") {
                div(class = "col s12") {
                    a(href = "/guilds", class = "breadcrumb"): "Guilds";
                    a(href = "#", class = "breadcrumb dropdown-trigger", data-target = "categories-dropdown"){
                        : guild.name;
                        : lib::icon("arrow-drop-down");
                    }
                    a(href = "#", class = "breadcrumb dropdown-trigger", data-target = "channels-dropdown"){
                        : current_channel.name;
                        : lib::icon("arrow-drop-down");
                    }
                }
                div {
                    div(id = "categories-dropdown", class = "dropdown-content") {
                        @ for group in groups {
                            a(href = "#") {
                                @ if let Some(cat) = &group.category {
                                    : cat.name;
                                } else {
                                    : "(no category)";
                                }
                            }
                        }
                    }
                    ul(id = "channels-dropdown", class = "dropdown-content") {
                        li {
                            @ for chan in current_group.channels {
                                a(href = format_args!("/guilds/{}/{}", guild.id, chan.id)): format_args!("#{}", chan.name);
                            }
                        }
                    }
                }
                ul(class = "sidenav sidenav-fixed", id = "side-channel-list") {
                    @ for cat in groups {
                        @ if let Some(cat) = &cat.category {
                            li {
                                a(href = "#"): cat.name;
                            }
                        }
                        @ for chan in cat.channels {
                            li {
                                a(href = format_args!("/guilds/{}/{}", guild.id, chan.id)): format_args!("#{}", chan.name);
                            }
                        }
                        li {
                            div(class = "divider") {}
                        }
                    }
                }
                a(href = "#", data-target = "side-channel-list"): lib::icon("menu");
            }
        },
        html! {
            div(class = "container") {
                div(class = "row") {
                    div(class = "col s12 m8 offset-ml x17 offset-xl1") {
                        // TODO styles
                        div(class = "wc-message-list") {
                            @ for message in messages {
                                div(class = "wc-message") {
                                    span(class = "wc-message");
                                    span(class = "wc-message-time") {
                                        : "[";
                                        span(class = "wc-message-hour"): message.time.0;
                                        : ":";
                                        span(class = "wc-message-hour"): message.time.0;
                                        : "] ";
                                    }
                                    span(
                                        class = "wc-message-author",
                                        data-user-id = message.author.id,
                                        data-user-avatar = message.author.avatar.unwrap_or("")
                                    ) {
                                        span(class = "wc-message-author-name"): message.author.name;
                                        @ if message.author.bot {
                                            : " ";
                                            span(class = "wc-message-author-bot indigo accent-1"): "BOT";
                                        }
                                    }
                                    span(class = "wc-message-colon"): ": ";
                                    span(class = "wc-message-content"): message.content;
                                    div(class = "wc-message-reactions") {
                                        @ for (emoji, count) in message.reactions {
                                            span(class = "wc-message-reaction") {
                                                img(class = "responsive-img", src = emoji.image_url().as_ref());
                                                span(class = "wc-mssage-reaction-count"): format_args!("{}", count);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div(class = "col hide-on-small-only m3 xl3 offset-xl1") {
                        h5: date.format("%A, %e %b, %Y").to_string();
                        div(id = "channel-date-picker");
                    }
                }
            }
        },
    )
}

#[derive(Debug)]
pub struct Guild<'t> {
    pub id: GuildId,
    pub name: &'t str,
}

#[derive(Debug)]
pub struct Group<'t, IChan>
where
    IChan: IntoIterator<Item = &'t Channel<'t>> + Copy,
{
    pub category: Option<Category<'t>>,
    pub channels: IChan,
}

#[derive(Debug)]
pub struct Category<'t> {
    pub id: CategoryId,
    pub name: &'t str,
}

#[derive(Debug)]
pub struct Channel<'t> {
    pub id: ChannelId,
    pub name: &'t str,
}

pub struct Message<'t, R, IReact>
where
    R: RenderOnce + 't,
    IReact: Iterator<Item = (Emoji<'t>, u64)>,
{
    pub time: (u32, u32, u32), // (hours, minutes, seconds)
    pub author: Author<'t>,
    pub content: R,
    pub reactions: IReact,
}

#[allow(dead_code)]
pub enum Emoji<'t> {
    Unicode(&'t str),
    Custom(EmojiId),
}

impl<'t> Emoji<'t> {
    fn image_url<'u>(&'u self) -> Cow<'static, str> {
        match self {
            Self::Unicode(id) => Cow::Borrowed(crate::EMOJI_LIST[*id].as_ref()),
            Self::Custom(snowflake) => Cow::Owned(format!(
                "https://cdn.discordapp.com/emojis/{}.png",
                *snowflake
            )),
        }
    }
}

pub struct Author<'t> {
    pub id: UserId,
    pub avatar: Option<&'t str>,
    pub name: &'t str,
    pub discrim: &'t str,
    pub bot: bool,
    // TODO: role info
}
