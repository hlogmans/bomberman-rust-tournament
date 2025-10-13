use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use bots::available_bots;
use game::bot::bot::{Bot};
use game::game::game::Game;
use gloo_timers::callback::Interval;
use crate::components::grid::Grid;
use gloo_timers::future::sleep;
use wasm_bindgen_futures::spawn_local;
use crate::components::player_icon::PlayerIcon;
use crate::components::run_game_result::RunGameResult;
use runner::tournament::*;
use runner::tournament_result::{Score, TournamentResult};
use std::time::Duration;

#[derive(Params, PartialEq)]
struct GameParams {
    bots: Option<String>,
}

#[component]
pub fn Game() -> impl IntoView {
    let query = use_query::<GameParams>();
    let bots = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.bots.as_ref())
            .map(|s| {
                s.split(',')
                 .filter_map(|x| x.parse::<usize>().ok())
                 .collect::<Vec<usize>>()
            })
            .unwrap_or_default()
    };
    let bots_vec = bots();

    let bot_constructors = available_bots();
    let a_idx = *bots_vec.get(0).unwrap_or(&0);
    let b_idx = *bots_vec.get(1).unwrap_or(&1);
    let bots_in_game: Vec<Box<dyn Bot>> =
        vec![bot_constructors[a_idx](), bot_constructors[b_idx]()];
    let bot_names = bots_in_game
        .iter()
        .map(|b| b.name().split_whitespace().next().unwrap().to_string())
        .collect::<Vec<_>>();

    let game_result = run_game(bots_in_game, 11);
   
    view! {
        {
                move || {
                    bot_names.iter().enumerate().map(|(i, name)| {
                        // For all but the last player, we’ll show a “VS” after their block
                        let has_next = i < bot_names.len() - 1;

                        view! {
                            <div class="flex flex-col items-center">
                                <div class="flex flex-row items-center justify-center space-x-2">
                                    <div class="w-16 h-16 object-contain">
                                        <PlayerIcon index={i} /> 
                                    </div>
                                    
                                    <p class="text-lg font-semibold text-center">{name.to_string()}</p>
                                </div>

                                {move || if has_next {
                                    view! {
                                        <p class="text-xl font-bold my-4">"VS"</p>
                                    }.into_any()
                                } else {
                                    view! {}.into_any()
                                }}
                            </div>
                        }
                    }).collect_view()
                }
            }
        <RunGameResult game_result=game_result/>
    }
}

