use leptos::*;
use leptos::prelude::*;
use super::player_icon::PlayerIcon;

#[component]
pub fn PlayerSection(players: Vec<String>) -> impl IntoView {
    view! {
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
    }
}

