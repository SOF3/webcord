use super::{html, lib, Args, Output};

pub fn render<'t>(
    Args {
        global,
        page,
        local,
    }: Args<'t, Local<'t>>,
) -> Output {
    lib::layout(
        global,
        page,
        html! {
            main {
                div(class = "container section") {
                    h3: page.title;
                    p(class = "light"): local.message;
                }
            }
        },
    )
}

#[derive(Debug, Clone)]
pub struct Local<'t> {
    pub message: &'t str,
}
