use leptos::prelude::*;


#[component]
pub fn SpeedControl(timer: ReadSignal<u64>, set_timer: WriteSignal<u64>) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3">
            <label for="speed-input" class="font-bold text-white">
                "Speed (ms):"
            </label>
            <input
                id="speed-input"
                type="number"
                class="w-24 bg-gray-900 text-white border border-gray-600 rounded-lg px-3 py-2 focus:outline-none focus:ring-4 focus:ring-blue-400 focus:border-blue-500 placeholder-gray-400"
                value=timer.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    if let Ok(v) = value.parse::<u64>() {
                        set_timer.set(v);
                    }
                }
                placeholder="250"
            />
        </div>
    }
}

