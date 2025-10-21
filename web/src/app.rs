use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
    path
};
use crate::pages::home::home_page::HomePage;
use crate::pages::game::game_config_page::GameConfigPage;
use crate::pages::game::game_run_page::GameRunPage;
use crate::pages::tournament::tournament_page::TournamentPage;


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/web.css"/>
        <Title text="BURP"/>
        <Router>
            <main 
                class="
                    flex flex-col items-center justify-center 
                    min-h-screen 
                    bg-gradient-to-br from-blue-400 via-blue-500 to-blue-600 
                    text-white font-mono
                "
            >
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("/game/config") view=GameConfigPage/>
                    <Route path=path!("/game/run") view=GameRunPage/>
                    <Route path=path!("/tournament") view=TournamentPage/>
                </Routes>
            </main>
        </Router>
    }
}
