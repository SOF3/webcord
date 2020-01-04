use horrorshow::Raw;

use super::{html, Critical, GlobalArgs, Output, PageArgs, PageConfig, Render, RenderOnce};

pub fn minimal_layout<'t, C: PageConfig>(
    global: &'t GlobalArgs,
    page: &'t PageArgs<'t, C>,
    main_block: impl RenderOnce + 't,
) -> Output {
    layout_impl(global, page, main_block, true)
}

pub fn layout<'t, C: PageConfig>(
    global: &'t GlobalArgs,
    page: &'t PageArgs<'t, C>,
    main_block: impl RenderOnce + 't,
) -> Output {
    layout_impl(global, page, main_block, false)
}

pub fn layout_impl<'t, C: PageConfig>(
    global: &'t GlobalArgs,
    page: &'t PageArgs<'t, C>,
    main_block: impl RenderOnce + 't,
    minimal: bool,
) -> Output {
    use horrorshow::{helper::doctype, Template};

    let render = html! {
        : doctype::HTML;

        html(lang = "en") {
            head {
                : head(global, &page);
            }

            body {
                : nav(global, &page, minimal);
                main: main_block;
                : foot(global, &page);
            }
        }
    };
    render.into_string().map_err(|err| {
        log::error!("Error rendering template: {}", err);
        Critical
    })
}

fn head<'t, C: PageConfig>(global: &'t GlobalArgs, page: &'t PageArgs<'t, C>) -> impl Render + 't {
    html! {
        title: page.title;
        meta(charset = "UTF-8");
        meta(http-equiv = "X-UA-Compatible", content = "chrome=1");
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
        script {
            : Raw("var WEBCORD_CILIENT_CONFIG = ");
            : Raw(serde_json::to_string(C::page_type()).expect("Failed to serialize PageConfig"));
        }
    }
}

fn nav<'t, C: PageConfig>(
    global: &'t GlobalArgs,
    page: &'t PageArgs<'t, C>,
    minimal: bool,
) -> impl Render + 't {
    html! {
        nav(role = "navigation", class = "light-green darken-4") {
            div(class = "nav-wrapper") {
                div(class = "container") {
                    a(id = "logo-container", href = "/", class = "brand-logo"): "webcord";
                    ul(class = "right hide-on-med-and-down"): side_nav(global, page, minimal);
                }
                a(href = "#", data-target = "mobile-menu", class = "sidenav-trigger") : icon("menu");
                ul(class = "sidenav", id = "mobile-menu"): side_nav(global, page, minimal);
            }
        }
    }
}

fn side_nav<'t, C: PageConfig>(
    _global: &'t GlobalArgs,
    page: &'t PageArgs<'t, C>,
    minimal: bool,
) -> impl Render + 't {
    html! {
        li {
            a(href = "/guilds") {
                : icon("view_list");
                : "Guilds";
            }
        }
        @ if !minimal {
            @ if let Some(_login) = page.login {
                li {
                    a(href = "/account") {
                        center: icon("account_circle");
                        : "Manage";
                    }
                }
                li {
                    a(href = "/logout") {
                        center: icon("power_settings_new");
                        : "Logout";
                    }
                    // TODO: Find a place to put : format_args!("{}#{}", &login.username, &login.discrim)
                }
            } else {
                li {
                    a(href = "/invite") {
                        center: icon("add");
                        : "Manage/Invite";
                    }
                }
            }
        }
    }
}

fn foot<'t, C: PageConfig>(global: &'t GlobalArgs, _page: &'t PageArgs<'t, C>) -> impl Render + 't {
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

fn icon(name: &'static str) -> impl Render {
    html! {
        i(class = "material-icons"): name;
    }
}
