use leptos::prelude::*;

use crate::components::bomberman_logo::BombermanLogo;
use crate::components::bot_selector::BotSelector;
use crate::components::start_game_link::StartGameLink;


#[component]
pub fn Home() -> impl IntoView {
    let (selected_bots, set_selected_bots) = signal(Vec::<usize>::new());
    provide_context(selected_bots);
    provide_context(set_selected_bots);

    view! {
        <div class="flex items-center justify-center gap-8 p-6">
            <BombermanLogo />
            <BotSelector />
        </div>
        <StartGameLink />
    }
}
