use leptos::*;
use leptos::prelude::*;
use bots::get_bot_names;

#[component]
pub fn BotSelector() -> impl IntoView {
    let bots = get_bot_names();

    // safely grab contexts
    let selected_bots = use_context::<ReadSignal<Vec<usize>>>()
        .expect("missing `selected_bots` context");
    let set_selected_bots = use_context::<WriteSignal<Vec<usize>>>()
        .expect("missing `set_selected_bots` context");

    view! {
        <div class="p-6">
            <h2 class="text-2xl font-bold text-center text-gray-200 mb-4 drop-shadow-lg">
                Choose Your Bots
            </h2>

            <ul class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4 justify-items-center">
                {bots
                    .into_iter()
                    .enumerate()
                    .map(|(i, bot)| {
                        let is_selected = Memo::new(move |_| selected_bots.get().contains(&i));

                        view! {
                            <BotOption 
                                bot=bot 
                                index=i
                                is_selected=is_selected
                                set_selected_bots=set_selected_bots
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
pub fn BotOption(
    bot: String,
    index: usize,
    is_selected: Memo<bool>,
    set_selected_bots: WriteSignal<Vec<usize>>,
) -> impl IntoView {
    view! {
        <li
            on:click=move |_| {
                set_selected_bots.update(|bots| {
                    if let Some(pos) = bots.iter().position(|x| *x == index) {
                        bots.remove(pos);
                    } else if bots.len() < 2 {
                        bots.push(index);
                    }
                });
            }
            class=move || format!(
                "px-4 py-3 rounded-2xl shadow-md text-center w-32 cursor-pointer select-none transition-all duration-300 {}",
                if is_selected.get() {
                    "bg-orange-500 text-white scale-105 shadow-orange-400/50"
                } else {
                    "bg-gray-800 text-gray-200 hover:bg-orange-500 hover:scale-105"
                }
            )
        >
            {bot}
        </li>
    }
}
