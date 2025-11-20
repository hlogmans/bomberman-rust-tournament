use leptos::prelude::*;
use gloo_timers::future::sleep;
use wasm_bindgen_futures::spawn_local;
use tournament::tournament::*;
use std::time::Duration;
use game::game::game_result::GameResult;

use super::grid::Grid;
use super::player_section::PlayerSection;
use super::round_control_buttons::RoundControlButtons;
use super::speed_control::SpeedControl;

#[component]
pub fn RunGameResult(game_result: GameResult) -> impl IntoView {
    let game_result_clone_payer_section = game_result.clone();
    let (game_replay, _set_replay) = signal(replay(&game_result));
    let (count, set_count) = signal(0);
    let (play, set_play) = signal(false);
    let (timer, set_timer) = signal(250);
    let (game_state, set_game_state) = signal(game_replay.get().turns[0].clone());
    
    Effect::new(move |_| {
        play.get();
        spawn_local(async move {
            let play = play.clone();
            while play.get() {
                set_count.set(count.get() + 1);
                sleep(Duration::from_millis(timer.get())).await;
            }
        });
    });

    Effect::new(move |_| {
        let effect_count = count.get();
        if effect_count > 0 && effect_count < game_replay.get().turns.len(){
            set_game_state.set(game_replay.get().turns[effect_count].clone());
        }else {
            set_play.set(false);
        }
    });

    view! {
        <div class="flex flex-col lg:flex-row items-center justify-center gap-10">
            <div class="flex flex-col items-center gap-4 text-white p-6">
                <div class="text-center space-y-1">
                    <p class="text-xl font-semibold">"Winner: " {game_result.winner}</p>
                    <div class="text-gray-300">"Score: " {game_result.score}</div>
                    <div class="text-gray-300">"Round: " {count}</div>
                </div>

                <Grid game_state=game_state width=game_result.game_settings.size/>
            </div>

            <div class="flex flex-col justify-between bg-gray-800/90 text-white rounded-2xl shadow-xl border border-gray-700 p-8 min-h-120">
                <PlayerSection game_result=game_result_clone_payer_section count=count game_state=game_state/>
                
                <div class="flex flex-col items-center gap-6 pt-8">
                    <RoundControlButtons play=play set_play=set_play set_count=set_count />
                    <SpeedControl timer=timer set_timer=set_timer />
                </div>
            </div>
        </div>
    }
}

