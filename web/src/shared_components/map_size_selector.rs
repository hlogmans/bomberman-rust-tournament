use leptos::*;
use leptos::prelude::*;

#[component]
pub fn MapSizeSelector() -> impl IntoView {
    let map_sizes = use_context::<Vec<usize>>()
        .expect("missing `map_sizes` context");
    let grid_size = use_context::<ReadSignal<usize>>()
        .expect("missing `grid_size` context");
    let set_grid_size = use_context::<WriteSignal<usize>>()
        .expect("missing `set_grid_size` context");

    view! { 
        <div class="flex flex-col items-center gap-2">
            <span class="text-lg font-bold text-center text-gray-200 drop-shadow-lg gap-2">"Choose your grid size"</span>
            <select
                on:change:target=move |ev| {
                    set_grid_size.set(ev.target().value().parse().unwrap());
                }
                prop:value=move || grid_size.get().to_string()
                class="bg-gray-800 text-white font-semibold rounded-2xl px-4 py-3 
                shadow-lg cursor-pointer border-2 border-gray-700 select-none 
                text-center transition-all duration-300 hover:scale-105 focus:outline-none"
            >
                {
                    map_sizes.iter().map(|size| {
                        view! {
                            <option
                                value=size.to_string()
                            >
                                {size.to_string()}
                            </option>
                        }
                    }).collect::<Vec<_>>()
                }
            </select>
        </div>
    }
}
