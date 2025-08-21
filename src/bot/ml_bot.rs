use crate::coord::{Col, Row};
use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Command, Map},
};

#[derive(Clone)]
pub struct MartijnBot {
    pub name: String,
    pub id: usize,
    map_settings: MapSettings,
}

impl MartijnBot {
    pub fn new(name: String) -> Self {
        MartijnBot {
            name,
            id: 0,
            map_settings: MapSettings::default(),
        }
    }

    fn compute_enemy_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = self.create_initial_enemy_heatmap(map);

        for _ in 0..3 {
            heatmap = self.propagate_heatmap(map, &heatmap);
        }
        self.normalize_vec(&mut heatmap);

        heatmap
    }
    fn create_initial_enemy_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = self.empty_heatmap();

        map.players
            .iter()
            .filter(|p| !p.name.contains(&self.name))
            .for_each(|p| {
                heatmap[self.idx(p.position.row.get(), p.position.col.get())] = 1.0;
            });

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
                        && self.is_clear(map, Self::make_coord(new_row, new_col))
                    {
                        let idx = self.idx(new_row, new_col as usize);
                        heatmap[idx] = bomb_heat; // Same heat for all tiles in blast
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
                            && self.is_clear(map, Self::make_coord(new_row, new_col))
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

    fn empty_heatmap(&self) -> Vec<f32> {
        vec![0.0; self.map_settings.width * self.map_settings.height]
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
        let enemy_heatmap = self.compute_enemy_heatmap(map);
        let blast_heatmap = self.create_blast_heatmap(map);
        for row in 0..map.height {
            for col in 0..map.width {
                print!("{:.2} ", blast_heatmap[self.idx(row, col)]);
            }
            println!();
        }
        println!();
        Command::Wait
    }
}
