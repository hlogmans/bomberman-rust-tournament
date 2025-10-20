use leptos::prelude::*;

#[component]
pub fn PlayerIcon(index: usize) -> impl IntoView {
    let player_image = match index {
        0 => "/images/red-crab.png",
        1 => "/images/blue-crab.png",
        2 => "/images/green-crab.png",
        3 => "/images/yellow-crab.png",
        _ => "/images/yellow-crab.png",
    };
    view! {
        <img
            src={player_image}
            alt="Player icon"
        />
    }
}

