use criterion::{criterion_group, criterion_main, Criterion};
use rust_bomberman::{tournament, bot::available_bots};
use rust_bomberman::tournament::prepare_bots;

fn tournament_benchmark(c: &mut Criterion) {
    c.bench_function("run_tournament", |b| {
        b.iter(|| {
            let bots = available_bots();
            let game_bots = prepare_bots(&bots);
            tournament::run_game(game_bots);
        });
    });

}

criterion_group!(benches, tournament_benchmark);
criterion_main!(benches);
