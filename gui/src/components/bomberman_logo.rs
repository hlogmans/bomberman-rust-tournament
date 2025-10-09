use leptos::*;
use leptos::prelude::*;

#[component]
pub fn BombermanLogo() -> impl IntoView {
    view! {
        <img src="/images/transparent-background-logo.png" class="w-64 h-64 object-cover" alt="Bomberman icon" />
    }
}
