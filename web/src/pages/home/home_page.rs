use leptos::prelude::*;
use crate::shared_components::bomberman_logo::BombermanLogo;
use crate::shared_components::link::Link;
use super::speech_bubble::SpeechBubble;
use super::speech_texts::speech_texts;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-6 p-6">
            <div class="relative">
                <BombermanLogo />
                <SpeechBubble texts=speech_texts()/>
            </div>
            <div class="flex gap-4">
                <Link text="Start tournament!".to_string() link="/tournament".to_string()/>
                <Link text="Start game!".to_string() link="/game/config".to_string()/>
            </div>
        </div>
    }
}