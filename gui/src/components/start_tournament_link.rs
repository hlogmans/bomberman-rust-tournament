use leptos::*;
use leptos::prelude::*;

#[component]
pub fn StartTournamentLink() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();
    
    view! {
        <button
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
                navigate(&format!("/tournament"),Default::default());
            }
        >
            "Fight all!"
        </button>
    }
}