use leptos::prelude::*;
use crate::server::execute_tournament::execute_new_tournament;
use crate::shared_components::game_runner::run_game_result::RunGameResult;
use crate::shared_components::game_runner::loading::Loading;

#[component]
pub fn TournamentPage() -> impl IntoView {
    view! {
        <Suspense
            fallback=move || view! { <Loading /> }
        >
            {move || Suspend::new(async move {
                let result = execute_new_tournament().await;
                let result_tournament = result.unwrap();
                let mut sorted_scores: Vec<_> = result_tournament.scores.iter().collect();
                sorted_scores.sort_by(|a, b| {
                    let a_wp = a.1.wins as f64 / a.1.total_games.max(1) as f64;
                    let b_wp = b.1.wins as f64 / b.1.total_games.max(1) as f64;
                    b_wp.partial_cmp(&a_wp).unwrap()
                });

                view! {
                    <div class="flex gap-8 items-start">
                        <div class="flex-1">
                            <h2 class="text-xl font-bold mb-2">
                                {format!("Final Scores after {} games:", result_tournament.total_games)}
                            </h2>
                            <ul class="list-disc pl-6">
                                {
                                    sorted_scores.iter().map(|(player, score)| {
                                        let win_pct = (score.wins as f64 / score.total_games.max(1) as f64) * 100.0;
                                        view! {
                                            <li>
                                                {format!(
                                                    "{}: Win%: {:.1} (W:{} / L:{} / G:{})",
                                                    player, win_pct, score.wins, score.losses, score.total_games
                                                )}
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </ul>
                        </div>
                        <div class="flex-none">
                            <h2 class="text-xl font-bold mb-2">
                                "Most interesting game:"
                            </h2>
                            <RunGameResult game_result=result_tournament.most_interesting.expect("Most interesting game")/>
                        </div>
                    </div>
                }
            })}
        </Suspense>
    }
}