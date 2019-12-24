use super::{html, lib, Args};

pub fn render(args: Args<'_, ()>) -> String {
    lib::layout(args, |args| {
        html! {<main>
            <div class="section no-pad-bot">
                <div class="container">
                    <h1 class="header center">"webcord"</h1>
                    <div class="row center">
                        <h5 class="header col s12 light">"Let everyone read chat logs on your Discord server without registering"</h5>
                    </div>
                    <div class="row center">
                        <a class="btn-large indigo accent-1" href={ &args.global.invite_link }>"Invite to Discord"</a>
                    </div> </div>
            </div>

            <div class="container section">
                <div class="row">
                    <div class="col s12 m4">
                        <div class="icon-block">
                            <h2 class="center"><i class="material-icons">"search"</i></h2>
                            <h5 class="center">"Optimized for search"</h5>
                            <p class="light">
                                "Just like forum pages, search engines can index chat history pages on webcord. \
                                Shy users can still use help from previous discussion without registering an account, \
                                and answers can be found by typing a query on a search engine to reduce duplicate questions."
                            </p>
                        </div>
                    </div>
                    <div class="col s12 m4">
                        <div class="icon-block">
                            <h2 class="center"><i class="material-icons">"wc"</i></h2>
                            <h5 class="center">"idk what's really so good that I can advertise"</h5>
                            <p class="light">""</p>
                        </div>
                    </div>
                    <div class="col s12 m4">
                        <div class="icon-block">
                            <h2 class="center"><i class="material-icons">"group"</i></h2>
                            <h5 class="center">"maybe let's just delete these columns?"</h5>
                            <p class="light">""</p>
                        </div>
                    </div>
                </div>
            </div>
        </main>}
    })
}
