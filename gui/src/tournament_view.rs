use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use futures::future::join_all;
use runner::tournament_result::TournamentResult;
use crate::tournament_worker::{TournamentWorker, TournamentInput};
use gloo_worker::Spawnable;
use crate::components::run_game_result::RunGameResult;

#[component]
pub fn Tournament() -> impl IntoView {
    let duration = 10000.0;
    let num_workers = 5;

    let (result_signal, set_result) = signal(None::<TournamentResult>);

    spawn_local(async move {
        let mut tasks = vec![];

        for _ in 0..num_workers {
            let mut bridge =
                TournamentWorker::spawner().spawn_with_loader("/tournament_worker_loader.js");

            let input = TournamentInput { duration };
            let task = async move { bridge.run(input).await };
            tasks.push(task);
        }

        let results = join_all(tasks).await;

        let mut grand_totals = TournamentResult::new();
        for mut res in results {
            grand_totals.merge_with(&mut res);
        }

        set_result.set(Some(grand_totals));
    });

    view! {
        <div class="p-4">
            {move || {
                if let Some(result_tournament) = result_signal.get() {
                    // Sort by win percentage
                    let mut sorted_scores: Vec<_> = result_tournament.scores.iter().collect();
                    sorted_scores.sort_by(|a, b| {
                        let a_wp = a.1.wins as f64 / a.1.total_games.max(1) as f64;
                        let b_wp = b.1.wins as f64 / b.1.total_games.max(1) as f64;
                        b_wp.partial_cmp(&a_wp).unwrap()
                    });

                    let total_games = result_tournament.total_games;

                    view! {

                        <div>
                            <h2 class="text-xl font-bold mb-2">
                                {format!("Final Scores after {} games:", total_games)}
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
                        <div class="flex w-max">
                            <RunGameResult game_result=result_tournament.most_interesting.expect("Most interesting game")/>
                        </div>

                    }.into_any()
                } else {
                    view! {
                        <p class="text-lg font-semibold">{format!("Running {} tournaments for {:.1} seconds", num_workers, duration / 1000.0)}</p>
                    }.into_any()
                }
            }}
        </div>
    }
}
