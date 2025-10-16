use leptos::prelude::*;

#[component]
pub fn RoundControlButtons(play: ReadSignal<bool>, set_play: WriteSignal<bool>, set_count: WriteSignal<usize>) -> impl IntoView {
    view! {
        <div class="flex items-center justify-center gap-4">
            <button    
                disabled=move || play.get()
                on:click=move |_| set_count.update(|count| *count = count.saturating_sub(1))
                class="w-14 h-14 bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all">
                { "<-" }
            </button>

            <button 
                on:click=move |_| set_play.update(|play| *play = !*play)
                class=move || {
                    let base = "w-16 h-16 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all";
                    if play.get() {
                        format!("{} bg-red-500 hover:bg-red-600 active:bg-red-700", base)
                    } else {
                        format!("{} bg-green-500 hover:bg-green-600 active:bg-green-700", base)
                    }
                }>
                { move || if play.get() { "⏸" } else { "▶" } }
            </button>

            <button 
                disabled=move || play.get()
                on:click=move |_| set_count.update(|count| *count = count.saturating_add(1))
                class="w-14 h-14 bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all">
                { "->" }
            </button>

            <button 
                disabled=move || play.get()
                on:click=move |_| set_count.set(0)
                class="w-14 h-14 bg-yellow-500 hover:bg-yellow-600 active:bg-yellow-700 text-white font-bold rounded-full shadow-md transform hover:scale-110 transition-all">
                { "↻" }
            </button>
        </div>
    }
}

