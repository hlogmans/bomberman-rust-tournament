use leptos::*;
use leptos::prelude::*;
use crate::components::player_icon::PlayerIcon;

#[component]
pub fn Tile(tile_type: char, player: Option<usize>, bomb: Option<usize>, explosion: Option<usize>) -> impl IntoView {
    // Tile base classes
    let class = match tile_type {
        'W' => "w-8 h-8 bg-black relative",
        '.' => "w-8 h-8 bg-[#8B4513] border-2 border-[#5C3317] relative
         before:content-[''] before:absolute before:inset-0 before:bg-[linear-gradient(45deg,transparent_45%,#5C3317_45%,#5C3317_55%,transparent_55%)]
         after:content-[''] after:absolute after:inset-0 after:bg-[linear-gradient(-45deg,transparent_45%,#5C3317_45%,#5C3317_55%,transparent_55%)]",

        _ => "w-8 h-8 bg-gray-300 relative",
    };

    view! {
        <div class={class}>
            // Bomb circle (if any)
            {bomb.map(|_| view! {
                <img src="/images/bomb.png" class="w-8 h-8 absolute" alt="Bomb" />
            })}

           {player.map(|p| {
                view! {
                    <div class="w-8 h-8 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
                        <PlayerIcon index={p} /> 
                    </div>
                }
            })}

            // Explosion overlay (if any)
            {explosion.map(|_| view! {
                <div class="absolute inset-0 rounded-full opacity-80
                    bg-[radial-gradient(circle,_#FFFF66_0%,_#FFD633_40%,_#FF6600_70%,_#FF3300_90%)] pointer-events-none"></div>
            })}
        </div>
    }
}
