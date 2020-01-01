use super::{html, lib, Args, Output};

pub fn render<'t>(args: Args<'t, Local<'t>>) -> Output {
    lib::layout(args, |global, page, local| {
        html! {
            main {
                div(class = "container section") {
                    h3: page.title;
                    p(class = "light"): local.message;
                }
            }
        }
    })
}

#[derive(Debug, Clone)]
pub struct Local<'t> {
    pub message: &'t str,
}
