use horrorshow::{html, Render, RenderOnce};

use super::{Args, Critical, GlobalArgs, Output, PageArgs};

pub fn layout<'t, T, R>(
    mut args: Args<'t, T>,
    main_block: fn(&GlobalArgs, &PageArgs<'t>, &mut T) -> R,
) -> Output
where
    T: 't,
    R: RenderOnce + 't,
{
    use horrorshow::{helper::doctype, Template};

    let Args {
        global,
        page,
        local,
    } = args;

    let render = html! {
        : doctype::HTML;

        html(lang = "en") {
            head {
                : head(global, &page);
            }

            body {
                : nav(global, &page);
                : main_block(global, &page, local);
                : foot(global, &page);
            }
        }
    };
    render.into_string().map_err(|err| {
        log::error!("Error rendering template: {}", err);
        Critical
    })
}

fn head<'t>(global: &'t GlobalArgs, page: &'t PageArgs<'t>) -> impl Render + 't {
    html! {
        title: &page.title;
        meta(charset = "UTF-8");
        meta(name = "viewport", content = "width=device-width, initial-scale=1, shrink-to-fit=no");
        meta(name = "description", content = page.description);
        meta(name = "keywords", content = "discord,chat,log,mirror,message,history");
        meta(name = "og:site_name", content = "webcord");
        meta(name = "og:image", content = format_args!("{}/favicon.ico", &global.domain));
        meta(name = "og:title", content = page.title);
        meta(name = "og:type", content = "website");
        meta(name = "og:url", content = &global.domain);
        meta(name = "twitter:card", content = "summary");
        meta(name = "twitter:title", content = page.title);
        meta(name = "twitter:description", content = page.description);
        meta(name = "theme-color", content  =  "#1A4D22");
        meta(name = "mobile-web-app-capable", content = "yes");
        meta(name = "apple-mobile-web-app-capable", content = "yes");
        link(type = "image/x-icon", rel = "icon", href = "/favicon.ico");
        meta(name = "mobile-web-app-capable", content = "yes");
        link(rel = "stylesheet", href = format_args!("/style.css?{}", &global.runtime_id));
    }
}

fn nav<'t>(global: &'t GlobalArgs, page: &'t PageArgs<'t>) -> impl Render + 't {
    html! {
        nav(role = "navigation", class = "light-green darken-4") {
            div(class = "nav-wrapper container") {
                a(id = "logo-container", href = "/", class = "brand-logo"): "webcord";
                ul(class = "right") {
                    li {
                        a(href = "/guilds") {
                            i(class = "material-icons"): "view_list";
                            : "Guilds";
                        }
                    }
                    li {
                        a(href = &global.invite_link) {
                            i(class = "material-icons"): "add";
                            : "Invite";
                        }
                    }
                }
            }
        }
    }
}

fn foot<'t>(global: &'t GlobalArgs, page: &'t PageArgs<'t>) -> impl Render + 't {
    html! {
        footer(class = "page-footer light-green darken-4") {
            div(class = "container") {
                ul(class = "inline-list") {
                    li {
                        a(class = "white-text", href = "/privacy"): "Privacy policy";
                    }
                    li {
                        a(class = "white-text", href = "https://github.com/SOF3/webcord"): "GitHub";
                    }
                }
            }

            div(class = "footer-copyright") {
                div(class = "container") {
                    : "All chat messages displayed via this website are retrieved from Discord. \
                        We do not permanently store any content other than derived ones that cannot be restored. \
                        Shall there be any copyright complaints, please refer to ";
                    a(href = "https://discordapp.com/terms"): "Discord Terms";
                    : ".";
                }
            }
        }

        script(src = format_args!("/script.js?{}", global.runtime_id)) {}
    }
}
