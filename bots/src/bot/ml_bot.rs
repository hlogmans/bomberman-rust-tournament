use std::collections::VecDeque;

use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;
use game::shrink::calculate_shrink_location;

#[derive(Clone)]
struct Node {
    pos_row: usize,
    pos_col: usize,
    time: usize,
    first_move: Option<Command>,
}



#[derive(Clone)]
pub struct MlBot {
    pub name: String,
    pub id: usize,
    map_settings: MapConfig,
    turn: usize,
    next_shrink_location: Option<Coord>,
}

impl MlBot {
    pub fn new() -> Self {
        MlBot {
            name: "MartijnBot".to_string(),
            id: 0,
            map_settings: MapConfig::default(),
            turn: 0,
            next_shrink_location: None,
        }
    }

    fn propagate_and_normalize(&self, map: &Map, mut heatmap: Vec<f32>) -> Vec<f32> {
        for _ in 0..5 {
            heatmap = self.propagate_heatmap(map, &heatmap);
        }
        self.normalize_vec(&mut heatmap);

        heatmap
    }

    fn create_enemy_heatmap(&self, map: &Map) -> Vec<f32> {
        //let mut heatmap = self.empty_heatmap();
        let mut heatmap = vec![0.01; self.map_settings.width * self.map_settings.height];



        map.players
            .iter()
            .filter(|p| !p.name.contains(&self.name))
            .for_each(|p| {
                heatmap[self.idx(p.position.row.get(), p.position.col.get())] = 1.0;
            });

        heatmap
    }
    fn create_breakable_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = self.empty_heatmap();

        for row in 0..map.height {
            for col in 0..map.width {
                let idx = self.idx(row, col);
                let cell = self.get_map_cell(row, col, map);
                if cell == '.' {
                    heatmap[idx] = 1.0;
                }
            }
        }

        heatmap
    }

    fn create_blast_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = self.empty_heatmap();

        for bomb in &map.bombs {
            let bomb_heat = 1.0 / (bomb.timer as f32);

            let bomb_row = bomb.position.row.get();
            let bomb_col = bomb.position.col.get();
            heatmap[self.idx(bomb_row, bomb_col)] = bomb_heat;

            for &(delta_row, delta_col) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                for distance in 1..=self.map_settings.bomb_radius {
                    let new_row = (bomb_row as isize + delta_row * distance as isize) as usize;
                    let new_col = (bomb_col as isize + delta_col * distance as isize) as usize;

                    if !self.out_of_bounds(new_row, new_col)
                        && !self.is_wall(map, new_row, new_col)
                    {
                        let idx = self.idx(new_row, new_col);
                        heatmap[idx] = bomb_heat;
                    }else {
                        break;
                    }
                }
            }
        }

        heatmap
    }

    fn propagate_heatmap(&self, map: &Map, heatmap: &Vec<f32>) -> Vec<f32> {
        let mut propagated_heatmap = heatmap.clone();

        for row in 0..map.height {
            for col in 0..map.width {
                let index = self.idx(row, col);
                let original_value = heatmap[index];
                if original_value > 0.0 {
                    for (delta_row, delta_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let new_row = (row as isize + delta_row) as usize;
                        let new_col = (col as isize + delta_col) as usize;

                        if !self.out_of_bounds(new_row, new_col)
                            && !self.is_wall(map, new_row, new_col)
                        {
                            let new_index = self.idx(new_row, new_col);
                            propagated_heatmap[new_index] += original_value * 0.25;
                        }
                    }
                    propagated_heatmap[index] += original_value * 0.25;
                }
            }
        }

        propagated_heatmap
    }
    fn normalize_vec(&self, heatmap: &mut Vec<f32>) {
        if let Some(&max_value) = heatmap.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            if max_value > 0.0 {
                for value in heatmap.iter_mut() {
                    *value /= max_value;
                }
            }
        }
    }

    fn find_escape_path(
        &self,
        map: &Map,
        start_row: usize,
        start_col: usize,
        bomb_heatmap: &Vec<f32>,
    ) -> Option<(Command, usize)> {
        let mut visited = vec![false; bomb_heatmap.len()];
        let mut queue = VecDeque::new();

        let start_idx = self.idx(start_row, start_col);
        if bomb_heatmap[start_idx] == 0.0 {
            return Some((Command::Wait, 0));
        }

        queue.push_back(Node {
            pos_row: start_row,
            pos_col: start_col,
            time: 0,
            first_move: None,
        });
        visited[start_idx] = true;

        while let Some(node) = queue.pop_front() {
            let idx = self.idx(node.pos_row, node.pos_col);

            if bomb_heatmap[idx] == 0.0 {
                return node.first_move.map(|dir| (dir, node.time));
            }

            for &(dr, dc, dir) in &[
                (-1, 0, Command::Up),
                (1, 0, Command::Down),
                (0, -1, Command::Left),
                (0, 1, Command::Right),
            ] {
                let nr_isize = node.pos_row as isize + dr;
                let nc_isize = node.pos_col as isize + dc;

                if nr_isize < 0 || nc_isize < 0 {
                    continue;
                }

                let nr = nr_isize as usize;
                let nc = nc_isize as usize;

                if self.out_of_bounds(nr, nc) {
                    continue;
                }

                if !self.is_clear(map, nr, nc) {
                    continue;
                }

                let new_idx = self.idx(nr, nc);
                if visited[new_idx] {
                    continue;
                }

                visited[new_idx] = true;
                queue.push_back(Node {
                    pos_row: nr,
                    pos_col: nc,
                    time: node.time + 1,
                    first_move: node.first_move.or(Some(dir)),
                });
            }
        }

        None
    }



    fn can_safely_place_bomb(&self, map: &Map, pos_row: usize, pos_col: usize, current_bomb_heatmap: &Vec<f32>) -> bool {
        let mut simulated_heatmap = current_bomb_heatmap.clone();
        let bomb_heat = 1.0;

        simulated_heatmap[self.idx(pos_row, pos_col)] = bomb_heat;

        for &(dr, dc) in &[(-1,0),(1,0),(0,-1),(0,1)] {
            for distance in 1..=self.map_settings.bomb_radius {
                let nr_isize = pos_row as isize + dr * distance as isize;
                let nc_isize = pos_col as isize + dc * distance as isize;

                if nr_isize < 0 || nc_isize < 0 { break; }
                let nr = nr_isize as usize;
                let nc = nc_isize as usize;
                if self.out_of_bounds(nr, nc) { break; }

                let cell = self.get_map_cell(nr, nc, map);
                if cell == 'W' { break; }

                simulated_heatmap[self.idx(nr, nc)] = bomb_heat;
            }
        }
        self.find_escape_path(map, pos_row, pos_col, &simulated_heatmap).is_some()
    }


    fn decide_move(&mut self, map: &Map, _player_location: Coord) -> Command {
        let enemy_heatmap = self.propagate_and_normalize(map, self.create_enemy_heatmap(map));
        let bomb_heatmap = self.create_blast_heatmap(map);
        let breakable_heatmap = self.propagate_and_normalize(map, self.create_breakable_heatmap(map));
        let player_row = _player_location.row.get();
        let player_col = _player_location.col.get();
        let current_index = self.idx(player_row, player_col);
        let escape_path: Option<(Command,usize)> = self.find_escape_path(map, player_row, player_col, &bomb_heatmap);

        if bomb_heatmap[current_index] > 0.0 {
            return if let Some((direction, _steps)) = escape_path {
                direction
            } else {
                Command::Wait
            }
        }

        let mut best_action = Command::Wait;
        let mut best_score = f32::MIN;

        for &(action, delta_row, delta_col) in &[
            (Command::Up, -1, 0),
            (Command::Down, 1, 0),
            (Command::Left, 0, -1),
            (Command::Right, 0, 1),
            (Command::Wait, 0, 0)
        ] {
            let new_row = (player_row as isize + delta_row) as usize;
            let new_col = (player_col as isize + delta_col) as usize;
            let is_wait = delta_row == 0 && delta_col == 0;
            if  !is_wait && !self.is_clear(map, new_row, new_col) {
                continue;
            }

            let idx = self.idx(new_row, new_col);


            let bomb_danger = bomb_heatmap[idx];
            let enemy_pressure = enemy_heatmap[idx];
            let breakable_heat = breakable_heatmap[idx];

            let escape_score = if let Some((_dir, steps)) = self.find_escape_path(
                map,
                new_row,
                new_col,
                &bomb_heatmap,
            ) {
                -(steps as f32)
            } else {
                -999.0
            };


            let score = -bomb_danger * 5.0
                + enemy_pressure * 2.0
                + escape_score * 1.5
                + breakable_heat * 1.0;

            if score > best_score {
                best_score = score;
                best_action = action;
            }
        }

        if bomb_heatmap[current_index] == 0.0 {
            let enemy_nearby = enemy_heatmap[current_index] > 0.8;
            let breakable_nearby =  breakable_heatmap[current_index] > 0.3;

            if enemy_nearby || breakable_nearby {
                if self.can_safely_place_bomb(map, player_row, player_col, &bomb_heatmap) {
                    return Command::PlaceBomb;
                }
            }
        }

        best_action
    }


    #[inline(always)]
    fn empty_heatmap(&self) -> Vec<f32> {
        vec![0.0; self.map_settings.width * self.map_settings.height]
    }

    #[inline(always)]
    fn is_wall(&self, map: &Map, row: usize, col: usize) -> bool {
        let cell = self.get_map_cell(row, col, map);
        cell == 'W' ||  cell == '.'
    }

    #[inline(always)]
    fn is_clear(&self, map: &Map, row: usize, col: usize) -> bool {
        self.get_map_cell(row, col, map) == ' ' && (self.next_shrink_location.is_none()  || (
            self.next_shrink_location.is_some()
            && self.next_shrink_location.unwrap().col.get() != col
            && self.next_shrink_location.unwrap().row.get() != row))
    }

    #[inline(always)]
    fn get_map_cell(&self, row: usize, col: usize, map: &Map) -> char {
        self.get_grid_value(&map.grid, row, col)
    }
    #[inline(always)]
    fn get_grid_value<T: Copy>(&self, grid: &Vec<T>, row: usize, col: usize) -> T {
        *grid
            .get(self.idx(row, col))
            .expect("Out of bounds")
    }

    #[inline(always)]
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.map_settings.width + col
    }
    #[inline(always)]
    fn out_of_bounds(&self, row: usize, col: usize) -> bool {
        row >= self.map_settings.height || col >= self.map_settings.width
    }
}

impl Bot for MlBot {
    fn name(&self) -> String {
        // return the name plus the ID
        format!("{} ({})", self.name, self.id)
    }

    fn get_debug_info(&self) -> String {
       "test".to_string()
    }

    fn start_game(&mut self, settings: &MapConfig, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = settings.clone();
        true
    }

    fn get_move(&mut self, map: &Map, _player_location: Coord) -> Command {
        if map.map_settings.endgame <= self.turn {
            self.next_shrink_location = None;
            if let Some(shrink_location) = calculate_shrink_location(
                self.turn - map.map_settings.endgame,
                map.map_settings.width,
                map.map_settings.height,
            ) {
                self.next_shrink_location = Some(shrink_location);
            }
        }
        self.turn = self.turn + 1;
        let next_move = self.decide_move(map, _player_location);
        next_move
    }
}
