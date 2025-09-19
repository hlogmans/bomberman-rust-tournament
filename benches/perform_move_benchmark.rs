use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rust_bomberman::game::map_settings::MapSettings;
use rust_bomberman::map::{Map, Command};

fn perform_move_benchmark(c: &mut Criterion) {
    let mut config = Criterion::default()
        .sample_size(1000) // default is 100; increase for more precision
        .measurement_time(std::time::Duration::from_secs(30)) // default is 5s
        .warm_up_time(std::time::Duration::from_secs(5)); // default is 3s

    let width = 9;
    let height = 9;
    let players = vec!["A".to_string(), "B".to_string()];

    let map_settings = MapSettings {
        bombtimer: 4,
        bombradius: 3,
        endgame: 500,
        width,
        height,
        playernames: Vec::new(),
    };

    let commands = vec![
        Command::Right,
        Command::PlaceBomb,
        Command::Left,
        Command::Down,
        Command::Up,
    ];

    let player_index = 0;

    config.bench_function("perform_move_sequence", |b| {
        b.iter(|| {
            let mut map = Map::create(width, height, players.clone(), map_settings.clone());

            for cmd in &commands {
                black_box(map.perform_move(player_index, *cmd));
            }
        });
    });
}


criterion_group!(perform_move_group, perform_move_benchmark);
criterion_main!(perform_move_group);
