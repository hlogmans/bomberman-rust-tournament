use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
    path
};

use crate::pages::home::home_page::HomePage;
use crate::pages::game::game_config_page::GameConfigPage;
use crate::pages::game::game_run_page::GameRunPage;
use crate::pages::tournament::tournament_page::TournamentPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Title text="BURP"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <main class="
                flex flex-col items-center justify-center 
                min-h-screen 
                bg-linear-to-br from-blue-400 via-blue-500 to-blue-600 
                text-white font-mono
            ">
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