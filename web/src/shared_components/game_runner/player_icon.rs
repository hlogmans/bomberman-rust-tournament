use leptos::prelude::*;

#[component]
pub fn PlayerIcon(index: usize, #[prop(optional)] is_dead: bool) -> impl IntoView {
    let player_image = match index {
        0 => "/images/red-crab.png",
        1 => "/images/blue-crab.png",
        2 => "/images/green-crab.png",
        3 => "/images/yellow-crab.png",
        _ => "/images/yellow-crab.png",
    };

    view! {
        <div class="relative w-full h-full">
            <img
                src={player_image}
                alt="Player icon"
                class={if !is_dead {
                    "w-full h-full object-contain"
                } else {
                    "w-full h-full object-contain filter grayscale brightness-75 opacity-60"
                }}
            />
            {if is_dead {
                view! {
                    <div class="absolute inset-0 flex items-center justify-center">
                        <div class="absolute w-[2px] h-full bg-red-600 rotate-45 origin-center left-1/2 top-0"></div>
                        <div class="absolute w-[2px] h-full bg-red-600 -rotate-45 origin-center left-1/2 top-0"></div>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}
