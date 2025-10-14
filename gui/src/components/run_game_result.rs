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
use leptos::{logging};
use leptos::html::Input;

#[derive(Params, PartialEq)]
struct GameParams {
    bots: Option<String>,
}

#[component]
pub fn RunGameResult(game_result: GameResult) -> impl IntoView {
    let (game_replay, set_replay) = signal(replay(&game_result));
    let (count, set_count) = signal(0);
    let (play, set_play) = signal(false);
    let (timer, set_timer) = signal(250);
    let (game_state, set_game_state) = signal(game_replay.get().turns[0].clone());
    
    create_effect(move |_| {
        play.get();
        spawn_local(async move {
            let play = play.clone();
            while play.get() {
                set_count.set(count.get() + 1);
                sleep(Duration::from_millis(timer.get())).await;
            }
        });
    });

    create_effect(move |_| {
        let effect_count = count.get();
        let test = effect_count > 0 || effect_count < game_replay.get().turns.len();
        if (effect_count > 0 && effect_count < game_replay.get().turns.len()){
            set_game_state.set(game_replay.get().turns[effect_count].clone());
        }else {
            set_play.set(false);
        }
    });

    view! {
        <div>
            <div>"Round: " {count}</div>
            <div class="flex w-max">
                <Grid game_state=game_state width=game_result.game_settings.width/>
            </div>
            <p>"Winner: " {game_result.winner}</p>
        </div><div class="flex flex-col items-center gap-6 mt-6 p-6 bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 rounded-2xl shadow-2xl">
    
<div class="flex items-center justify-center gap-4">
    <button    
        disabled=move || play.get()
        on:click=move |_| set_count.set((count.get().saturating_sub(1)).max(0))
        class="w-14 h-14 bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold rounded-full shadow-lg transform hover:scale-110 transition-all">
        { "<-" }
    </button>
    <button 
        on:click=move |_| set_play.update(|play| *play = !*play)
        class=move || {
            let base = "w-16 h-16 text-white font-bold rounded-full shadow-lg transform hover:scale-110 transition-all";
            if play.get() {
                format!("{} bg-red-500 hover:bg-red-600 active:bg-red-700", base) // Paused color
            } else {
                format!("{} bg-green-500 hover:bg-green-600 active:bg-green-700", base) // Playing color
            }
        }>
        { move || if play.get() { "⏸" } else { "▶" } }
    </button>
    <button 
        disabled=move || play.get()
        on:click=move |_| set_count.set(count.get() + 1)
        class="w-14 h-14 bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold rounded-full shadow-lg transform hover:scale-110 transition-all">
        { "->" }
    </button>
    <button 
        disabled=move || play.get()
        on:click=move |_| set_count.set(0)
        class="w-14 h-14 bg-yellow-500 hover:bg-yellow-600 active:bg-yellow-700 text-white font-bold rounded-full shadow-lg transform hover:scale-110 transition-all">
        { "↻" }
    </button>
</div>

<div class="flex items-center gap-3">
        <label for="speed-input" class="text-white font-bold">
            "Speed (ms):"
        </label>
        <input
            id="speed-input"
            type="number"
            class="w-24 bg-gray-800 text-white border-2 border-gray-600 rounded-lg px-3 py-2 focus:outline-none focus:ring-4 focus:ring-yellow-400 focus:border-yellow-500 placeholder-gray-400"
            value=timer.get()
            on:input=move |ev| {
                let value = event_target_value(&ev);
                if let Ok(v) = value.parse::<u64>() {
                    set_timer.set(v);
                }
            }
            placeholder="250"
        />
    </div>
</div>

    }
}

