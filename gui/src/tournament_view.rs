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
use runner::tournament::*;
use runner::tournament_result::{Score, TournamentResult};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;


#[component]
pub fn Tournament() -> impl IntoView {

    let counter = Arc::new(AtomicUsize::new(0));

    let bot_constructors = available_bots();
    let result_tournament = run_tournament_wasm(&bot_constructors, Some(counter), 10000.0);

    view! {
<ul>
        {
            result_tournament.scores.iter().map(|(player, score)| {
                view! {
                    <li>
                        {format!("{} - Wins: {}, Losses: {}, Total Games: {}", 
                                 player, score.wins, score.losses, score.total_games)}
                    </li>
                }
            }).collect::<Vec<_>>()
        }
    </ul>
    }
}

