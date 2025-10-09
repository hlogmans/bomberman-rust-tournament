use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::home::Home;
use crate::game::Game;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Title text="Bomberman"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <main class="
                flex flex-col items-center justify-center 
                min-h-screen 
                bg-gradient-to-br from-blue-400 via-blue-500 to-blue-600 
                text-white font-mono
            ">
                <Routes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=Home/>
                    <Route path=StaticSegment("game") view=Game/>
                </Routes>
            </main>
        </Router>
    }
}

