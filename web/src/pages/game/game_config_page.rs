use leptos::prelude::*;
use crate::shared_components::bot_selector::BotSelector;
use crate::shared_components::bomberman_logo::BombermanLogo;
use crate::shared_components::link::Link;
use crate::shared_components::map_size_selector::MapSizeSelector;

#[component]
pub fn GameConfigPage() -> impl IntoView {
    let (selected_bots, set_selected_bots) = signal::<Vec<usize>>(Vec::new());
    let map_sizes = odd_numbers_in_range(7, 20);
    let (grid_size, set_grid_size) = signal(map_sizes[2]);

    provide_context(selected_bots);
    provide_context(set_selected_bots);
    provide_context(map_sizes);
    provide_context(grid_size);
    provide_context(set_grid_size);

view! {
    <div class="flex items-center justify-center w-full max-w-5xl gap-12">
        <div class="flex flex-col items-center gap-4">
            <BombermanLogo />
            {move || {
                let size = grid_size.get();
                let count = selected_bots.get().len();
                let disabled = count < 2 || count > 4;
                let bots_param = selected_bots
                    .get()
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                view! {
                    <Link 
                        text="Start!".to_string() 
                        link=format!("/game/run?bots={bots_param}&size={size}") 
                        is_disabled=disabled 
                    />
                }
            }}
        </div>
        <div class="flex flex-col justify-center gap-4">
            <BotSelector />
            <MapSizeSelector />
        </div>
    </div>
}







}

fn odd_numbers_in_range(start: usize, end: usize) -> Vec<usize> {
    (start..=end).filter(|x| x % 2 == 1).collect()
}