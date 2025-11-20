use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::sleep;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::rng;

#[component]
pub fn SpeechBubble(texts: Vec<&'static str>) -> impl IntoView {
    let (displayed_text, set_displayed_text) = signal(String::new());
    spawn_local({
        let (text_index, set_text_index) = signal(0);
        let mut rng = rng();
        let mut shuffled_texts = texts.clone();
        shuffled_texts.shuffle(&mut rng);
        async move {
            let max = texts.len();
            loop {
                let idx = text_index.get();
                let full_text = shuffled_texts[idx];
                set_displayed_text.set(String::new());
                
                for c in full_text.chars() {
                    set_displayed_text.update(|s| s.push(c));
                    sleep(Duration::from_millis(50)).await;
                }
                sleep(Duration::from_secs(2)).await;
                set_text_index.update(|i| *i = (*i + 1) % max);
            }
        }
    });

    view! {
        <div class="absolute top-1/2 left-full ml-4 -translate-y-1/2 bg-white text-gray-800 p-3 rounded-xl shadow-lg min-w-[180px]">
            { move || displayed_text.get() }
            <div class="w-3 h-3 bg-white absolute left-[-6px] top-1/2 -translate-y-1/2 rotate-45"></div>
        </div>
    }
}
