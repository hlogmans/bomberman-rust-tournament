use std::collections::VecDeque;
use crate::coord::{Col, Row};
use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Command, Map},
};

#[derive(Clone)]
struct Node {
    pos: Coord,
    time: usize,
    first_move: Option<Command>,
}



#[derive(Clone)]
pub struct MartijnBot {
    pub name: String,
    pub id: usize,
    map_settings: MapSettings,
}

impl MartijnBot {
    pub fn new() -> Self {
        MartijnBot {
            name: "Martijn".to_string(),
            id: 0,
            map_settings: MapSettings::default(),
        }
    }

    fn propagate_and_normalize(&self, map: &Map, mut heatmap: Vec<f32>) -> Vec<f32> {
        for _ in 0..3 {
            heatmap = self.propagate_heatmap(map, &heatmap);
        }
        self.normalize_vec(&mut heatmap);

        heatmap
    }

    fn create_enemy_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = self.empty_heatmap();

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
                let cell = self.get_map_cell(Self::make_coord(row, col), map);
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
                for distance in 1..=self.map_settings.bombradius {
                    let new_row = (bomb_row as isize + delta_row * distance as isize) as usize;
                    let new_col = (bomb_col as isize + delta_col * distance as isize) as usize;

                    if !self.out_of_bounds(new_row, new_col)
                        && !self.is_wall(map, Self::make_coord(new_row, new_col))
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
                            && !self.is_wall(map, Self::make_coord(new_row, new_col))
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
        start: Coord,
        bomb_heatmap: &Vec<f32>,
    ) -> Option<(Command, usize)> {
        let mut visited = vec![false; bomb_heatmap.len()];
        let mut queue = VecDeque::new();

        let start_idx = self.idx(start.row.get(), start.col.get());
        if bomb_heatmap[start_idx] == 0.0 {
            return Some((Command::Wait, 0));
        }

        queue.push_back(Node {
            pos: start,
            time: 0,
            first_move: None,
        });
        visited[start_idx] = true;

        while let Some(node) = queue.pop_front() {
            let idx = self.idx(node.pos.row.get(), node.pos.col.get());

            if bomb_heatmap[idx] == 0.0 {
                return node.first_move.map(|dir| (dir, node.time));
            }

            for &(dr, dc, dir) in &[
                (-1, 0, Command::Up),
                (1, 0, Command::Down),
                (0, -1, Command::Left),
                (0, 1, Command::Right),
            ] {
                let nr_isize = node.pos.row.get() as isize + dr;
                let nc_isize = node.pos.col.get() as isize + dc;

                if nr_isize < 0 || nc_isize < 0 {
                    continue;
                }

                let nr = nr_isize as usize;
                let nc = nc_isize as usize;

                if self.out_of_bounds(nr, nc) {
                    continue;
                }

                let new_coord = Self::make_coord(nr, nc);
                if !self.is_clear(map, new_coord) {
                    continue;
                }

                let new_idx = self.idx(nr, nc);
                if visited[new_idx] {
                    continue;
                }

                visited[new_idx] = true;
                queue.push_back(Node {
                    pos: new_coord,
                    time: node.time + 1,
                    first_move: node.first_move.or(Some(dir)),
                });
            }
        }

        None
    }


    fn breakable_in_range(&self, map: &Map, pos: Coord) -> bool {
        for &(dr, dc) in &[(-1,0),(1,0),(0,-1),(0,1)] {
            for step in 1..=self.map_settings.bombradius {
                let nr_isize = pos.row.get() as isize + dr * step as isize;
                let nc_isize = pos.col.get() as isize + dc * step as isize;

                if nr_isize < 0 || nc_isize < 0 {
                    break;
                }

                let nr = nr_isize as usize;
                let nc = nc_isize as usize;

                if self.out_of_bounds(nr, nc) {
                    break;
                }

                let cell_tpe = self.get_map_cell(Self::make_coord(nr, nc), map);

                if cell_tpe == 'W' { break; }
                if cell_tpe == '.' { return true; }
            }
        }
        false
    }

    fn can_safely_place_bomb(&self, map: &Map, pos: Coord, current_bomb_heatmap: &Vec<f32>) -> bool {
        let mut simulated_heatmap = current_bomb_heatmap.clone();
        let bomb_heat = 1.0;

        let row = pos.row.get();
        let col = pos.col.get();

        simulated_heatmap[self.idx(row, col)] = bomb_heat;

        for &(dr, dc) in &[(-1,0),(1,0),(0,-1),(0,1)] {
            for distance in 1..=self.map_settings.bombradius {
                let nr_isize = row as isize + dr * distance as isize;
                let nc_isize = col as isize + dc * distance as isize;

                if nr_isize < 0 || nc_isize < 0 { break; }
                let nr = nr_isize as usize;
                let nc = nc_isize as usize;
                if self.out_of_bounds(nr, nc) { break; }

                let cell = self.get_map_cell(Self::make_coord(nr, nc), map);
                if cell == 'W' { break; }

                simulated_heatmap[self.idx(nr, nc)] = bomb_heat;
            }
        }
        self.find_escape_path(map, pos, &simulated_heatmap).is_some()
    }


    fn decide_move(&mut self, map: &Map, _player_location: Coord) -> Command {
        let enemy_heatmap = self.propagate_and_normalize(map, self.create_enemy_heatmap(map));
        let bomb_heatmap = self.create_blast_heatmap(map);
        let breakable_heatmap = self.propagate_and_normalize(map, self.create_breakable_heatmap(map));
        let current_index = self.idx(_player_location.row.get(), _player_location.col.get());
        let escape_path: Option<(Command,usize)> = self.find_escape_path(map, _player_location, &bomb_heatmap);

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
            let new_row = (_player_location.row.get() as isize + delta_row) as usize;
            let new_col = (_player_location.col.get() as isize + delta_col) as usize;
            let is_wait = delta_row == 0 && delta_col == 0;
            if  !is_wait && !self.is_clear(map, Self::make_coord(new_row, new_col)) {
                continue;
            }

            let idx = self.idx(new_row, new_col);


            let bomb_danger = bomb_heatmap[idx];
            let enemy_pressure = enemy_heatmap[idx];
            let breakable_heat = breakable_heatmap[idx];

            let escape_score = if let Some((_dir, steps)) = self.find_escape_path(
                map,
                Self::make_coord(new_row, new_col),
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
            let breakable_nearby =  breakable_heatmap[current_index] > 0.4; //self.breakable_in_range(map, _player_location);

            if enemy_nearby || breakable_nearby {
                if self.can_safely_place_bomb(map, _player_location, &bomb_heatmap) {
                    return Command::PlaceBomb;
                }
            }
        }

        best_action
    }


    fn empty_heatmap(&self) -> Vec<f32> {
        vec![0.0; self.map_settings.width * self.map_settings.height]
    }

    fn is_wall(&self, map: &Map, coord: Coord) -> bool {
        let cell = self.get_map_cell(coord, map);
        cell == 'W' ||  cell == '.'
    }
    fn is_clear(&self, map: &Map, coord: Coord) -> bool {
        self.get_map_cell(coord, map) == ' '
    }
    fn make_coord(row: usize, col: usize) -> Coord {
        Coord::new(Col::new(col), Row::new(row))
    }
    fn get_map_cell(&self, coord: Coord, map: &Map) -> char {
        self.get_grid_value(&map.grid, coord)
    }
    fn get_grid_value<T: Copy>(&self, grid: &Vec<T>, location: Coord) -> T {
        *grid
            .get(self.idx(location.row.get(), location.col.get()))
            .expect("Out of bounds")
    }
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.map_settings.width + col
    }
    fn out_of_bounds(&self, row: usize, col: usize) -> bool {
        row >= self.map_settings.height || col >= self.map_settings.width
    }
}

impl Bot for MartijnBot {
    fn name(&self) -> String {
        // return the name plus the ID
        format!("{} ({})", self.name, self.id)
    }

    fn start_game(&mut self, settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = settings.clone();
        true
    }

    fn get_move(&mut self, map: &Map, _player_location: Coord) -> Command {
        let next_move = self.decide_move(map, _player_location);
        next_move

    }
}
