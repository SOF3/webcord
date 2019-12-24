use super::{html, lib, text, Args};

pub fn render<'t>(args: Args<'t, Local<'t>>) -> String {
    lib::layout(args, |args| {
        html! {<main>
            <div class="container section">
                <h3 class="">{ text!("{}", args.page.title) }</h3>
                <p class="light">{ text!("{}", args.local.message) }</p>
            </div>
        </main>}
    })
}

#[derive(Debug, Clone)]
pub struct Local<'t> {
    pub message: &'t str,
}
