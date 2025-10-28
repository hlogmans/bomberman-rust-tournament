use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use runner::tournament::*;
use game::bot::bot::{BotController};
use bots::available_bots;
use crate::shared_components::game_runner::run_game_result::RunGameResult;


#[derive(Params, PartialEq, Clone)]
struct GameParams {
    bots: Option<String>,
    size: Option<usize>,
}
#[component]
pub fn GameRunPage() -> impl IntoView {
    let query = use_query::<GameParams>();

    let game_result = Resource::new(
        move || query.read().clone(),
        move |params_opt| async move {
            
            let bot_constructors = available_bots();
            let params = params_opt.expect("no params");
            let size = params.size.unwrap_or(11);
            let bots = params.bots.as_ref()
                .map(|s: &String| {
                    s.split(',')
                     .filter_map(|x| x.parse::<usize>().ok())
                     .map(|i| bot_constructors[i]())
                     .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            run_game(bots, size)
        }
    );

    view! {
        <Suspense
            fallback=move || view! { <p class="text-white">"Loading game..."</p> }
        >
            {move || Suspend::new(async move {
                let result = game_result.get();
                view! {
                    <RunGameResult game_result=result.expect("suspensed")/>
                }
            })}
        </Suspense>
    }
}
