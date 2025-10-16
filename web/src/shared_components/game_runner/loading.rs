use leptos::*;
use leptos::prelude::*;
use super::grid::Grid;
use game::map::player::Player;
use game::coord::Coord;
use game::game::replay_engine::MapReplaySnapshot;

#[component]
pub fn Loading() -> impl IntoView {
    let animation = generate_loading_animation();

    #[allow(unused_variables)]
    let (count, set_count) = signal(0);

    #[allow(unused_variables)]
    let (game_state, set_game_state) = signal(animation[0].clone());

    #[allow(unused_variables)]
    let (displayed_text, set_displayed_text) = signal(String::new());

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::spawn_local;
        use gloo_timers::future::sleep;
        use std::time::Duration;
        spawn_local(async move {

            while count.get() < animation.len() {
                    set_count.set(count.get() + 1);
                    set_game_state.set(animation[count.get()].clone());
                sleep(Duration::from_millis(150)).await;
            }
        });

    }

    view! {
        <p>"Loading..."</p>
        <Grid game_state=game_state width=19/>    
    }
}

pub fn generate_loading_animation() -> Vec<MapReplaySnapshot> {
    let frames = 160;
    let mut snapshots = Vec::new();
    let mut explosions = Vec::new();

    let grid = vec![
        'W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',' ','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W',' ','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',' ','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W',' ','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',' ','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W',' ','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',' ','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W',' ','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',
        'W',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','W',
        'W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W',
    ];

    let player_1_path = path_to_center_player1();
    let player_2_path = path_to_center_player2();


    for turn in 0..frames {
        let player_1_pos = if turn < player_1_path.len() {
            player_1_path[turn].clone()
        } else {
            player_1_path.last().unwrap().clone()
        };

        let player_2_pos = if turn < player_2_path.len() {
            player_2_path[turn].clone()
        } else {
            player_2_path.last().unwrap().clone()
        };

        snapshots.push(MapReplaySnapshot {
            turn,
            players: vec![
                Player { name: "Alice".to_string(), position: player_1_pos },
                Player { name: "Bob".to_string(), position: player_2_pos },
            ],
            bombs: vec![],
            grid: grid.clone(),
            explosions: explosions.clone(),
        });
        explosions.push(player_1_pos);
        explosions.push(player_2_pos);
        // grid[player_1_pos.row.get() * size + player_1_pos.col.get()] = '.';
        // grid[player_2_pos.row.get() * size + player_2_pos.col.get()] = '.';
    }

    snapshots
}



pub fn path_to_center_player1() -> Vec<Coord> {
    vec![
        Coord::from(1,1), Coord::from(2,1), Coord::from(3,1), Coord::from(4,1),
        Coord::from(5,1), Coord::from(6,1), Coord::from(7,1), Coord::from(8,1),
        Coord::from(9,1), Coord::from(10,1), Coord::from(11,1), Coord::from(12,1),
        Coord::from(13,1), Coord::from(14,1), Coord::from(15,1), Coord::from(16,1),
        Coord::from(17,1),Coord::from(17,2),

        Coord::from(17,3), Coord::from(16,3), Coord::from(15,3), Coord::from(14,3),
        Coord::from(13,3), Coord::from(12,3), Coord::from(11,3), Coord::from(10,3),
        Coord::from(9,3), Coord::from(8,3), Coord::from(7,3), Coord::from(6,3),
        Coord::from(5,3), Coord::from(4,3), Coord::from(3,3), Coord::from(2,3),
        Coord::from(1,3),Coord::from(1,4),

        Coord::from(1,5), Coord::from(2,5), Coord::from(3,5), Coord::from(4,5),
        Coord::from(5,5), Coord::from(6,5), Coord::from(7,5), Coord::from(8,5),
        Coord::from(9,5), Coord::from(10,5), Coord::from(11,5), Coord::from(12,5),
        Coord::from(13,5), Coord::from(14,5), Coord::from(15,5), Coord::from(16,5),
        Coord::from(17,5),Coord::from(17,6),

        Coord::from(17,7), Coord::from(16,7), Coord::from(15,7), Coord::from(14,7),
        Coord::from(13,7), Coord::from(12,7), Coord::from(11,7), Coord::from(10,7),
        Coord::from(9,7), Coord::from(8,7), Coord::from(7,7), Coord::from(6,7),
        Coord::from(5,7), Coord::from(4,7), Coord::from(3,7), Coord::from(2,7),
        Coord::from(1,7),Coord::from(1,8),

        Coord::from(1,9), Coord::from(2,9), Coord::from(3,9), Coord::from(4,9),
        Coord::from(5,9), Coord::from(6,9), Coord::from(7,9), Coord::from(8,9),
        Coord::from(9,9),
    ]
}


pub fn path_to_center_player2() -> Vec<Coord> {
    vec![
        Coord::from(17,17), Coord::from(16,17), Coord::from(15,17), Coord::from(14,17),
        Coord::from(13,17), Coord::from(12,17), Coord::from(11,17), Coord::from(10,17),
        Coord::from(9,17), Coord::from(8,17), Coord::from(7,17), Coord::from(6,17),
        Coord::from(5,17), Coord::from(4,17), Coord::from(3,17), Coord::from(2,17),
        Coord::from(1,17),Coord::from(1,16),

        Coord::from(1,15), Coord::from(2,15), Coord::from(3,15), Coord::from(4,15),
        Coord::from(5,15), Coord::from(6,15), Coord::from(7,15), Coord::from(8,15),
        Coord::from(9,15), Coord::from(10,15), Coord::from(11,15), Coord::from(12,15),
        Coord::from(13,15), Coord::from(14,15), Coord::from(15,15), Coord::from(16,15),
        Coord::from(17,15),Coord::from(17,14),

        Coord::from(17,13), Coord::from(16,13), Coord::from(15,13), Coord::from(14,13),
        Coord::from(13,13), Coord::from(12,13), Coord::from(11,13), Coord::from(10,13),
        Coord::from(9,13), Coord::from(8,13), Coord::from(7,13), Coord::from(6,13),
        Coord::from(5,13), Coord::from(4,13), Coord::from(3,13), Coord::from(2,13),
        Coord::from(1,13),Coord::from(1,12),

        Coord::from(1,11), Coord::from(2,11), Coord::from(3,11), Coord::from(4,11),
        Coord::from(5,11), Coord::from(6,11), Coord::from(7,11), Coord::from(8,11),
        Coord::from(9,11), Coord::from(10,11), Coord::from(11,11), Coord::from(12,11),
        Coord::from(13,11), Coord::from(14,11), Coord::from(15,11), Coord::from(16,11),
        Coord::from(17,11),Coord::from(17,10),

        Coord::from(17,9), Coord::from(16,9), Coord::from(15,9), Coord::from(14,9),
        Coord::from(13,9), Coord::from(12,9), Coord::from(11,9), Coord::from(10,9),
        Coord::from(9,9), 
    ]
}

