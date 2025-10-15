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
    
    let players = game_result.game_settings.player_names;


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
<div class="flex flex-col lg:flex-row items-center justify-center gap-10 p-8">
    <div class="flex flex-col items-center gap-4 text-white p-6">
        <div class="text-center space-y-1">
            <p class="text-xl font-semibold">"Winner: " {game_result.winner}</p>
            <div class="text-gray-300">"Round: " {count}</div>
        </div>

        <Grid game_state=game_state width=game_result.game_settings.width/>
    </div>

    <div class="flex flex-col justify-between bg-gray-800/90 text-white rounded-2xl shadow-xl border border-gray-700 p-8"
         style="height: 30rem;">
        <div class="flex flex-col items-start gap-3 w-full">
            {
                move || {
                    players.iter().enumerate().map(|(i, name)| {
                        view! {
                            <div class="flex items-center gap-3">
                                <div class="flex items-center justify-center w-10 h-10 rounded-full bg-blue-100 text-gray-800 shadow-sm">
                                    <PlayerIcon index={i} />
                                </div>
                                <p class="text-base font-medium leading-none">{name.to_string()}</p>
                            </div>
                        }
                    }).collect_view()
                }
            }
        </div>
        <div class="flex flex-col items-center gap-6">
            <div class="flex items-center justify-center gap-4">
                <button    
                    disabled=move || play.get()
                    on:click=move |_| set_count.set((count.get().saturating_sub(1)).max(0))
                    class="w-14 h-14 bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all">
                    { "<-" }
                </button>

                <button 
                    on:click=move |_| set_play.update(|play| *play = !*play)
                    class=move || {
                        let base = "w-16 h-16 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all";
                        if play.get() {
                            format!("{} bg-red-500 hover:bg-red-600 active:bg-red-700", base)
                        } else {
                            format!("{} bg-green-500 hover:bg-green-600 active:bg-green-700", base)
                        }
                    }>
                    { move || if play.get() { "⏸" } else { "▶" } }
                </button>

                <button 
                    disabled=move || play.get()
                    on:click=move |_| set_count.set(count.get() + 1)
                    class="w-14 h-14 bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all">
                    { "->" }
                </button>

                <button 
                    disabled=move || play.get()
                    on:click=move |_| set_count.set(0)
                    class="w-14 h-14 bg-yellow-500 hover:bg-yellow-600 active:bg-yellow-700 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all">
                    { "↻" }
                </button>
            </div>

            <div class="flex items-center gap-3">
                <label for="speed-input" class="font-bold text-white">
                    "Speed (ms):"
                </label>
                <input
                    id="speed-input"
                    type="number"
                    class="w-24 bg-gray-900 text-white border border-gray-600 rounded-lg px-3 py-2 focus:outline-none focus:ring-4 focus:ring-blue-400 focus:border-blue-500 placeholder-gray-400"
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
    </div>
</div>







    }
}

