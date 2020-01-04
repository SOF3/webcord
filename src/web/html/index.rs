use super::{html, lib, GlobalArgs, Output, PageArgs, PageConfig};

pub fn render<'t, C: PageConfig>(global: &'t GlobalArgs, page: PageArgs<'t, C>) -> Output {
    lib::layout(
        global,
        &page,
        html! {
            div(class = "section no-pad-bot") {
                div(class = "container") {
                    h1(class = "header center"): "webcord";
                    div(class = "row center") {
                        h5(class = "header col s12 light"):
                            "Let everyone read chat logs on your \
                                Discord server without registering";
                    }
                    div(class = "row center") {
                        a(class = "btn-large indigo accent-1", href = "/invite"): "Invite to Discord";
                    }
                }
            }

            div(class = "container section") {
                div(class = "row") {
                    div(class = "col s12 m6") {
                        div(class = "icon-block") {
                            h2(class = "center") {
                                i(class = "material-icons"): "search";
                            }
                            h5(class = "center"): "Optimized for search";
                            p(class = "light"): "\
                                Just like forum pages, search engines can index chat history pages on webcord. \
                                Shy users can still use help from previous discussion without registering an account, \
                                and answers can be found by typing a query on a search engine to reduce duplicate questions.";
                        }
                    }
                    div(class = "col s12 m6") {
                        div(class = "icon-block") {
                            h2(class = "center") {
                                i(class = "material-icons"): "wc";
                            }
                            h5(class = "center"): "idk what's really so good that I can advertise";
                            p(class = "light"): "Let's just write crap here."
                        }
                    }
                }
            }
        },
    )
}
