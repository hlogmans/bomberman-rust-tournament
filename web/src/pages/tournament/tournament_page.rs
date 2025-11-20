use gloo_net::http::Request;
use leptos::web_sys::RequestMode;
use leptos::prelude::*;
use tournament::tournament_result::TournamentResult;
use crate::shared_components::game_runner::run_game_result::RunGameResult;
use crate::shared_components::game_runner::loading::Loading;

#[component]
pub fn TournamentPage() -> impl IntoView {
    let data = LocalResource::new(move || async move {
        let response = Request::get("http://localhost:3200/tournament/run")
            .mode(RequestMode::Cors) 
            .send()
            .await
            .map_err(|err| err.to_string())?;

        let json = response
            .json::<TournamentResult>()
            .await
            .map_err(|_| "Failed to parse JSON".to_string());

        json
    });

    view! {
        <Suspense fallback=move || view! { <Loading /> }>
            {move || {
                match data.get() {
                    None => view! { <Loading /> }.into_any(), // still loading
                    Some(Err(_err)) => view! { <p>"Something went wong"</p><p>{_err}</p> }.into_any(), // fetch/parsing error
                    Some(Ok(result)) => {
                        let mut sorted_scores: Vec<_> = result.scores.iter().collect();
                        sorted_scores.sort_by(|a, b| {
                            let a_wp = a.1.wins as f64 / a.1.total_games.max(1) as f64;
                            let b_wp = b.1.wins as f64 / b.1.total_games.max(1) as f64;
                            b_wp.partial_cmp(&a_wp).unwrap()
                        });

                        view! {
                            <div class="flex gap-8 items-start">
                                <div class="flex-1">
                                    <h2 class="text-xl font-bold mb-2">
                                        {format!("Final Scores after {} games:", result.total_games)}
                                    </h2>
                                    <ul class="list-disc pl-6">
                                        {sorted_scores.iter().map(|(player, score)| {
                                            let win_pct = (score.wins as f64 / score.total_games.max(1) as f64) * 100.0;
                                            view! {
                                                <li>
                                                    {format!(
                                                        "{}: Win%: {:.1} (W:{} / L:{} / G:{})",
                                                        player, win_pct, score.wins, score.losses, score.total_games
                                                    )}
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </div>
                                <div class="flex-none">
                                    <h2 class="text-xl font-bold mb-2">
                                        "Most interesting game:"
                                    </h2>
                                    {
                                        if let Some(game) = &result.most_interesting {
                                            view! { <RunGameResult game_result=game.clone() /> }.into_any()
                                        } else {
                                            view! { <p>"No interesting game available"</p> }.into_any()
                                        }
                                    }
                                </div>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </Suspense>
    }
}
