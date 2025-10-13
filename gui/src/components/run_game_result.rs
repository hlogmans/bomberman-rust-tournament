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
use std::time::Duration;
use game::game::game_result::GameResult;

#[derive(Params, PartialEq)]
struct GameParams {
    bots: Option<String>,
}

#[component]
pub fn RunGameResult(game_result: GameResult) -> impl IntoView {
    let game_replay = replay(&game_result);
    let (count, set_count) = signal(0);
    let (game_state, set_game_state) = signal(game_replay.turns[0].clone());
    let (winner_signal, set_winner) = signal(None::<String>);

    spawn_local(async move {
        for round in game_replay.turns {
            set_count.set(count.get() + 1);
            set_game_state.set(round.clone());
            sleep(Duration::from_millis(250)).await;

        }
        set_winner.set(Some(game_result.winner.to_string()));
    });

    view! {
        <div>
            <div>"Round: " {count}</div>
            <Grid game_state=game_state width=game_result.game_settings.width/>
            {move || {
                if let Some(winner) = winner_signal.get() {
                    view! { <p>"Winner: " {winner}</p> }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}

