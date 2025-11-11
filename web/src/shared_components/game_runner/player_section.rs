use leptos::*;
use leptos::prelude::*;
use super::player_icon::PlayerIcon;
use game::game::game_result::GameResult;
use game::map::player::Player;
use game::game::replay_engine::MapReplaySnapshot;

#[component]
pub fn PlayerSection(game_result: GameResult, count: ReadSignal<usize>, game_state: ReadSignal<MapReplaySnapshot>) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full gap-4">
            {
                move || {
                    let count_value = count.get();
                    let players = game_state.get().players;
                    players
                        .iter()
                        .map(|player: &Player| {
                            let max_index = game_result.replay_data[player.id].len().saturating_sub(1);
                            let safe_index = count_value.min(max_index);

                            let command = &game_result.replay_data[player.id][safe_index];
                            let debug_info = &game_result.debug_data[player.id][safe_index];

                            view! {
                                <div class="flex flex-col w-full p-4 rounded-2xl shadow-md bg-gray-700/90 border border-gray-600 transition-colors duration-150 hover:bg-gray-600/90">
                                    <div class="flex items-center gap-3">
                                        <div class="flex items-center justify-center w-10 h-10 rounded-full bg-white text-gray-800 shadow-sm">
                                            <PlayerIcon index={player.id} is_dead=!player.is_alive() />
                                        </div>
                                        <p class="text-lg font-semibold text-gray-100">{player.name.to_string()}</p>
                                    </div>

                                    <div class="mt-2 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-1 text-sm text-gray-300">
                                        <p class="flex items-center">
                                            <span class="font-medium text-gray-100 mr-1">Command:</span>
                                            <span class="text-blue-300 font-semibold inline-block w-[7rem] truncate">
                                                {format!("{:?}", command)}
                                            </span>
                                        </p>
                                        {if player.is_alive() && !debug_info.is_empty() {
                                            view! {
                                                <p class="font-mono text-xs text-gray-400 bg-gray-800/50 rounded px-2 py-1">
                                                    {debug_info.to_string()}
                                                </p>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}
                                        {if !player.is_alive() {
                                            view! {
                                                <p class="font-mono text-xs text-gray-400 bg-gray-800/50 rounded px-2 py-1">
                                                    {player.reason_killed.to_string()}
                                                </p>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()
                }
            }
        </div>
    }
}



