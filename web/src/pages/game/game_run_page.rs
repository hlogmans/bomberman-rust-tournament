use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use runner::tournament::*;
use game::bot::bot::{Bot};
use bots::available_bots;
use crate::shared_components::game_runner::run_game_result::RunGameResult;


#[derive(Params, PartialEq)]
struct GameParams {
    bots: Option<String>,
    size: Option<usize>,
}
#[component]
pub fn GameRunPage() -> impl IntoView {
    let query = use_query::<GameParams>();
    let size: usize = query
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.size)
        .filter(|s| s % 2 == 1)
        .unwrap_or(11);
    let bot_constructors = available_bots();
    let bots_in_game = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.bots.as_ref())
            .map(|s| {
                s.split(',')
                 .filter_map(|x| x.parse::<usize>().ok())
                 .map(|index| bot_constructors[index]())
                 .collect::<Vec<Box<dyn Bot>>>()
            })
            .unwrap_or_default()
    };
    
    let game_result = run_game(bots_in_game(), size);

    view! {
        <RunGameResult game_result=game_result/>
    }
}
