use leptos::*;
use leptos::prelude::*;

#[component]
pub fn StartGameLink() -> impl IntoView {
    let selected_bots = use_context::<ReadSignal<Vec<usize>>>()
        .expect("No selected_bots context found");
    let navigate = leptos_router::hooks::use_navigate();
    let is_disabled = move || selected_bots.get().len() != 2;

    view! {
        <button
            disabled=move || is_disabled()
            class="
                inline-block
                mt-6
                bg-white
                text-blue-800
                font-semibold
                px-6
                py-3
                rounded-xl
                shadow-md
                hover:bg-gray-300
                hover:shadow-lg
                transition
                duration-300
                border-2
                border-blue-800
                disabled:opacity-50
                disabled:cursor-not-allowed
                cursor-pointer
            "
            on:click=move |_| {
                if !is_disabled() {
                    let bots_param = selected_bots
                        .get()
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    navigate(&format!("/game?bots={bots_param}"),Default::default());
                }
            }
        >
            "Fight!"
        </button>
    }
}