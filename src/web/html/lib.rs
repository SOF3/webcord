use typed_html::elements::FlowContent;
pub use typed_html::{html, text};

use super::Args;

pub fn layout<'t, T, F>(mut args: Args<'t, T>, main_block: F) -> String
where
    T: 't,
    F: FnOnce(&'_ mut Args<'t, T>) -> Box<dyn FlowContent<String> + 'static>,
{
    let dom = html! {
        <html lang="en">
            <head>
                <title> { text!("{}", &args.page.title) } </title>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no"/>
                <meta name="description" content={ args.page.description }/>
                <meta name="keywords" content="discord,chat,log,mirror,message,history"/>
                <meta name="og:site_name" content="webcord"/>
                <meta name="og:image" content={ &format!("{}/favicon.ico", &args.global.domain) }/>
                <meta name="og:title" content={ args.page.title }/>
                <meta name="og:type" content="website"/>
                <meta name="og:url" content={ &args.global.domain }/>
                <meta name="twitter:card" content="summary"/>
                <meta name="twitter:title" content={ args.page.title }/>
                <meta name="twitter:description" content={ args.page.description }/>
                <meta name="theme-color" content="#1A4D22"/>
                <meta name="mobile-web-app-capable" content="yes"/>
                <meta name="apple-mobile-web-app-capable" content="yes"/>
                <link type="image/x-icon" rel="icon" href="/favicon.ico"/>
                <meta name="mobile-web-app-capable" content="yes"/>
                <link rel="stylesheet" href={ &format!("/style.css?{}", &args.global.runtime_id)}/>
            </head>

            <body>
                <nav role="navigation" class="light-green darken-4">
                    <div class="nav-wrapper container">
                        <a id="logo-container" href="/" class="brand-logo">"webcord"</a>
                        <ul class="right">
                            <li>
                                <a href="/guilds">
                                    <i class="material-icons">"view_list"</i>
                                    "Guilds"
                                </a>
                            </li>
                            <li>
                                <a href={ &args.global.invite_link }>
                                    <i class="material-icons">"add"</i>
                                    "Invite"
                                </a>
                            </li>
                        </ul>
                    </div>
                </nav>

                { main_block(&mut args) }

                <footer class="page-footer light-green darken-4">
                    <div class="container">
                        <ul class="inline-list">
                            <li>
                                <a class="white-text" href="/privacy">"Privacy policy"</a>
                            </li>
                            <li>
                                <a class="white-text" href="https://github.com/SOF3/webcord">"GitHub"</a>
                            </li>
                        </ul>
                    </div>

                    <div class="footer-copyright">
                        <div class="container">
                            "All chat messages displayed via this website are retrieved from Discord. \
                            We do not permanently store any content other than derived ones that cannot be restored. \
                            Shall there be any copyright complaints, please refer to "
                            <a href="https://discordapp.com/terms">"Discord Terms"</a> "."
                        </div>
                    </div>
                </footer>

                <script src={format!("/script.js?{}", args.global.runtime_id)}></script>
            </body>
        </html>
    };
    dom.to_string()
}
