use leptos::*;
use leptos::prelude::*;
use crate::components::tile::Tile;
use game::game::game_state::GameState;

#[component]
pub fn Grid(game_state: ReadSignal<GameState>) -> impl IntoView {
    let width = 11;

    view! {
        <div
            class="grid grid-cols-(--my-grid-cols) border-white border-2"
            style=move || format!("--my-grid-cols: repeat({}, minmax(0, 1fr));", width)
        >
            { move || {
                    let state = game_state.get();
                    let player_indexes: Vec<usize> = state.players.into_iter().map(|p| p.position.row.get() * width + p.position.col.get()).collect();
                    let bomb_indexes: Vec<usize> = state.bombs.into_iter().map(|b| b.position.row.get() * width + b.position.col.get()).collect();
                    let explosion_indexes: Vec<usize> = state.explosions.into_iter().map(|e| e.row.get() * width + e.col.get()).collect();
                    state.grid.iter().enumerate().map(|(i, t)| view! {
                        <Tile 
                            tile_type={*t} 
                            player={player_indexes.iter().position(|&pi| pi == i)}
                            bomb={bomb_indexes.iter().position(|&bi| bi == i)}
                            explosion={explosion_indexes.iter().position(|&ei| ei == i)}
                            />
                    }).collect_view()
                }
            }
        </div>
    }
}
