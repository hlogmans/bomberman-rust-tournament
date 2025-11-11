use std::collections::VecDeque;
use std::fmt::Write;
use std::sync::Arc;

use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::shrink::calculate_shrink_location;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};

const INPUT_SIZE: usize = 14;
const HIDDEN_SIZE: usize = 8;
const ACTION_DIM: usize = 6;

#[derive(Clone, Debug)]
pub struct NeuralWeights {
    pub w1: [[f32; INPUT_SIZE]; HIDDEN_SIZE],
    pub b1: [f32; HIDDEN_SIZE],
    pub w2: [f32; HIDDEN_SIZE],
    pub b2: f32,
}

impl Default for NeuralWeights {
    fn default() -> Self {
        NeuralWeights {
    w1: [
        [1.778051, -5.793366, 1.528579, 1.199841, -0.095680, -2.108942, -1.364485, -1.764208, -2.355936, 2.776789, -0.119606, -1.306864, -3.713537, 3.271143],
        [1.273877, -0.426565, 2.597434, 3.319680, 0.621564, 1.711760, 3.458726, 2.400491, -0.455454, 5.567872, -1.468685, 0.617420, -2.931117, -0.445271],
        [0.300977, -2.749179, -0.890582, 0.869030, 2.079161, 4.812849, -1.452201, 1.283057, 1.472435, -2.331308, 2.158853, 1.108746, 0.651509, -1.615412],
        [0.271131, -4.905676, 1.426842, -0.713114, -0.572197, -1.106308, 3.373036, 1.740779, 2.102649, 0.665267, -1.292585, -0.072332, 1.061783, 4.793095],
        [-0.171796, -0.385991, 1.222988, 3.947078, 2.160987, 0.879065, 0.017882, -0.887031, 1.176665, 1.866714, 0.196925, 1.116174, 1.560331, 0.146115],
        [-4.036108, -1.000531, -0.862752, 1.865028, 2.113852, 2.962975, -2.261739, 3.576754, 3.580922, 3.790398, 0.898554, 2.758185, 0.519205, 0.884854],
        [-0.246541, 0.797280, -1.279504, 1.989421, 5.091067, -6.031856, -0.446350, 4.454270, 1.517469, 3.862473, -0.266528, -0.645484, -0.861274, -0.419883],
        [1.568220, -3.185035, -1.502080, 3.468473, 2.569712, -1.432129, 3.271850, -0.130087, 1.102846, 3.906012, 2.283769, 0.210173, 1.788464, -1.100726],
    ],
    b1: [1.122302, 3.937719, 0.806253, 1.728679, 2.503488, 1.511190, 0.779869, 0.539155],
    w2: [0.978115, 2.781071, 4.591185, 3.558485, 0.239106, -1.652016, -2.213315, 3.886446],
    b2: 1.218848,
}
    }
}

impl NeuralWeights {
    pub fn random(rng: &mut impl Rng) -> Self {
        let mut weights = Self {
            w1: [[0.0; INPUT_SIZE]; HIDDEN_SIZE],
            b1: [0.0; HIDDEN_SIZE],
            w2: [0.0; HIDDEN_SIZE],
            b2: 0.0,
        };

        for row in weights.w1.iter_mut() {
            for value in row.iter_mut() {
                *value = Distribution::<f32>::sample(&StandardNormal, rng);
            }
        }

        for value in weights.b1.iter_mut() {
            *value = Distribution::<f32>::sample(&StandardNormal, rng);
        }

        for value in weights.w2.iter_mut() {
            *value = Distribution::<f32>::sample(&StandardNormal, rng);
        }

        weights.b2 = Distribution::<f32>::sample(&StandardNormal, rng);
        weights
    }

    pub fn perturb(&self, rng: &mut impl Rng, sigma: f32) -> Self {
        let mut next = self.clone();
        for row in next.w1.iter_mut() {
            for value in row.iter_mut() {
                let noise: f32 = Distribution::<f32>::sample(&StandardNormal, rng) * sigma;
                *value += noise;
            }
        }
        for value in next.b1.iter_mut() {
            let noise: f32 = Distribution::<f32>::sample(&StandardNormal, rng) * sigma;
            *value += noise;
        }
        for value in next.w2.iter_mut() {
            let noise: f32 = Distribution::<f32>::sample(&StandardNormal, rng) * sigma;
            *value += noise;
        }
        next.b2 += Distribution::<f32>::sample(&StandardNormal, rng) * sigma;
        next
    }

    pub fn format_as_rust(&self) -> String {
        let mut output = String::new();
        output.push_str("NeuralWeights {\n    w1: [\n");
        for row in &self.w1 {
            output.push_str("        [");
            for (idx, value) in row.iter().enumerate() {
                if idx > 0 {
                    output.push_str(", ");
                }
                write!(&mut output, "{:.6}", value).unwrap();
            }
            output.push_str("],\n");
        }
        output.push_str("    ],\n    b1: [");
        for (idx, value) in self.b1.iter().enumerate() {
            if idx > 0 {
                output.push_str(", ");
            }
            write!(&mut output, "{:.6}", value).unwrap();
        }
        output.push_str("],\n    w2: [");
        for (idx, value) in self.w2.iter().enumerate() {
            if idx > 0 {
                output.push_str(", ");
            }
            write!(&mut output, "{:.6}", value).unwrap();
        }
        output.push_str("],\n    b2: ");
        write!(&mut output, "{:.6}", self.b2).unwrap();
        output.push_str(",\n}\n");
        output
    }
}

const ACTIONS: [Command; 6] = [
    Command::Up,
    Command::Down,
    Command::Left,
    Command::Right,
    Command::Wait,
    Command::PlaceBomb,
];

#[derive(Clone)]
struct Node {
    pos_row: usize,
    pos_col: usize,
    time: usize,
    first_move: Option<Command>,
}

#[derive(Clone)]
struct NeuralNetwork {
    weights: Arc<NeuralWeights>,
}

impl NeuralNetwork {
    fn new(weights: Arc<NeuralWeights>) -> Self {
        Self { weights }
    }

    fn forward(&self, input: &[f32; INPUT_SIZE]) -> f32 {
        let mut hidden = [0.0_f32; HIDDEN_SIZE];
        let weights = self.weights.as_ref();
        for i in 0..HIDDEN_SIZE {
            let mut acc = weights.b1[i];
            for j in 0..INPUT_SIZE {
                acc += weights.w1[i][j] * input[j];
            }
            hidden[i] = acc.max(0.0);
        }

        let mut output = weights.b2;
        for i in 0..HIDDEN_SIZE {
            output += weights.w2[i] * hidden[i];
        }

        output
    }

    fn weights(&self) -> Arc<NeuralWeights> {
        Arc::clone(&self.weights)
    }
}

#[derive(Clone)]
pub struct NeuralBot {
    name: String,
    id: usize,
    map_settings: MapConfig,
    turn: usize,
    next_shrink_location: Option<Coord>,
    network: NeuralNetwork,
    last_debug: String,
    command_list: Vec<Command>,
    current_index: usize,
    looping: bool,
    initialized: bool,
}

impl NeuralBot {
    pub fn new() -> Self {
        Self::with_weights(Arc::new(NeuralWeights::default()), "NeuralNemesis".to_string())
    }

    fn hardcoded_script() -> Vec<Command> {
        vec![
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Down,
            Command::Wait,
            Command::Up,
            Command::Right,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Down,
            Command::Wait,
            Command::Up,
            Command::Right,
            Command::Right,
            Command::Right,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            Command::Right,
            // bomb place and run
            //30
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            Command::Right,
            Command::Down,
            Command::Down,
            // bomb place and run
            Command::PlaceBomb,
            Command::Up,
            Command::Up,
            Command::Left,
            Command::Right,
            Command::Down,
            Command::Down,
            Command::Down,
            Command::Down,
            // bomb place and run
            Command::PlaceBomb,
            Command::Up,
            Command::Up,
            Command::Right,
            Command::Left,
            Command::Down,
            Command::Down,
            // bomb place and run
            Command::PlaceBomb,
            Command::Up,
            Command::Up,
            Command::Left,
        ]
    }

    pub fn with_weights(weights: Arc<NeuralWeights>, label: String) -> Self {
        NeuralBot {
            name: label,
            id: 0,
            map_settings: MapConfig::default(),
            turn: 0,
            next_shrink_location: None,
            network: NeuralNetwork::new(weights),
            last_debug: String::new(),
            command_list: vec![],
            current_index: 0,
            looping: false,
            initialized: false,
        }
    }

    pub fn weights(&self) -> Arc<NeuralWeights> {
        self.network.weights()
    }

    fn decide_move(&mut self, map: &Map, player_location: Coord) -> Command {
        let enemy_heatmap = self.propagate_and_normalize(map, self.create_enemy_heatmap(map));
        let breakable_heatmap = self.propagate_and_normalize(map, self.create_breakable_heatmap(map));
    let danger_heatmap = self.create_danger_heatmap(map);

        let player_row = player_location.row.get();
        let player_col = player_location.col.get();
        let current_idx = self.idx(player_row, player_col);

        if danger_heatmap[current_idx] > 0.0 {
            if let Some((escape_dir, _)) = self.find_escape_path(map, player_row, player_col, &danger_heatmap) {
                self.last_debug = format!("forced_escape:{escape_dir:?}");
                return escape_dir;
            }
        }

        let mut best_action = Command::Wait;
        let mut best_score = f32::MIN;
        let mut logs = Vec::new();

        for action in ACTIONS.iter().copied() {
            if let Some((score, log_entry)) = self.evaluate_action(
                map,
                player_row,
                player_col,
                action,
                &enemy_heatmap,
                &breakable_heatmap,
                &danger_heatmap,
            ) {
                logs.push(format!("{action:?}:{score:.3}::{log_entry}"));
                if score > best_score {
                    best_score = score;
                    best_action = action;
                }
            }
        }

        if logs.is_empty() {
            self.last_debug = "no-action".to_string();
            Command::Wait
        } else {
            self.last_debug = logs.join(" | ");
            best_action
        }
    }

    fn evaluate_action(
        &self,
        map: &Map,
        row: usize,
        col: usize,
        action: Command,
        enemy_heatmap: &Vec<f32>,
        breakable_heatmap: &Vec<f32>,
        danger_heatmap: &Vec<f32>,
    ) -> Option<(f32, String)> {
        if matches!(action, Command::PlaceBomb) && self.get_map_cell(row, col, map) == 'B' {
            return None;
        }

        let (target_row, target_col) = self.apply_action(row, col, action)?;

        if !matches!(action, Command::Wait | Command::PlaceBomb) && !self.is_clear(map, target_row, target_col) {
            return None;
        }

        let (local_danger, escape_steps) = if matches!(action, Command::PlaceBomb) {
            let simulated = self.simulate_bomb(map, danger_heatmap, row, col);
            match self.find_escape_path(map, row, col, &simulated) {
                Some((_, steps)) => (simulated, Some(steps)),
                None => return None,
            }
        } else {
            (
                danger_heatmap.clone(),
                self.find_escape_path(map, target_row, target_col, danger_heatmap)
                    .map(|(_, steps)| steps),
            )
        };

        let features = self.build_features(
            map,
            row,
            col,
            target_row,
            target_col,
            action,
            enemy_heatmap,
            breakable_heatmap,
            &local_danger,
            escape_steps,
        );

        let score = self.network.forward(&features);
        let log_entry = format!(
            "d_now={:.2},d_next={:.2},esc={:.2},aggr={:.2},bomb={:.2}",
            features[0], features[1], features[2], features[3], features[6]
        );
        Some((score, log_entry))
    }

    fn build_features(
        &self,
        map: &Map,
        row: usize,
        col: usize,
        target_row: usize,
        target_col: usize,
        action: Command,
        enemy_heatmap: &Vec<f32>,
        breakable_heatmap: &Vec<f32>,
        danger_heatmap: &Vec<f32>,
        escape_steps: Option<usize>,
    ) -> [f32; INPUT_SIZE] {
        let idx_current = self.idx(row, col);
        let idx_target = self.idx(target_row, target_col);

        let danger_here = danger_heatmap[idx_current];
        let danger_target = danger_heatmap[idx_target];
        let escape_quality = escape_steps
            .map(|steps| {
                let capped = (steps as f32).min(8.0);
                1.0 - (capped / 8.0)
            })
            .unwrap_or(0.0);

        let enemy_pressure = enemy_heatmap[idx_target];
        let breakable_pressure = breakable_heatmap[idx_target];
        let center_bias = self.center_bias(target_row, target_col);
        let kill_potential = if matches!(action, Command::PlaceBomb) {
            self.bomb_kill_score(map, row, col)
        } else {
            self.adjacent_enemy_score(map, target_row, target_col)
        };
        let mobility_score = self.safe_neighbor_ratio(map, target_row, target_col, danger_heatmap);

        let mut encoded = [0.0_f32; INPUT_SIZE];
        encoded[0] = danger_here;
        encoded[1] = danger_target;
        encoded[2] = escape_quality;
        encoded[3] = enemy_pressure;
        encoded[4] = breakable_pressure;
        encoded[5] = center_bias;
        encoded[6] = kill_potential;
        encoded[7] = mobility_score;

        let action_one_hot = self.encode_action(action);
        encoded[8..8 + ACTION_DIM].copy_from_slice(&action_one_hot);

        encoded
    }

    fn simulate_bomb(&self, map: &Map, danger_heatmap: &Vec<f32>, row: usize, col: usize) -> Vec<f32> {
        let mut simulated = danger_heatmap.clone();
        let idx = self.idx(row, col);
        simulated[idx] = 1.0;

        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in deltas {
            for distance in 1..=self.map_settings.bomb_radius {
                let nr_isize = row as isize + dr * distance as isize;
                let nc_isize = col as isize + dc * distance as isize;
                if nr_isize < 0 || nc_isize < 0 {
                    break;
                }
                let nr = nr_isize as usize;
                let nc = nc_isize as usize;

                if self.out_of_bounds(nr, nc) {
                    break;
                }

                let cell = self.get_map_cell(nr, nc, map);
                simulated[self.idx(nr, nc)] = 1.0;

                if cell == 'W' || cell == '.' {
                    break;
                }
            }
        }

        simulated
    }

    fn propagate_and_normalize(&self, map: &Map, mut heatmap: Vec<f32>) -> Vec<f32> {
        for _ in 0..4 {
            heatmap = self.propagate_heatmap(map, &heatmap);
        }
        self.normalize_vec(&mut heatmap);
        heatmap
    }

    fn create_enemy_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = vec![0.01; self.map_settings.size * self.map_settings.size];

        map.get_alive_players()
            .iter()
            .filter(|p| p.id != self.id)
            .for_each(|p| {
                heatmap[self.idx(p.position.row.get(), p.position.col.get())] = 1.0;
            });

        heatmap
    }

    fn create_breakable_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = self.empty_heatmap();
        for row in 0..self.map_settings.size {
            for col in 0..self.map_settings.size {
                let idx = self.idx(row, col);
                if self.get_map_cell(row, col, map) == '.' {
                    heatmap[idx] = 1.0;
                }
            }
        }
        heatmap
    }

    fn create_danger_heatmap(&self, map: &Map) -> Vec<f32> {
        let mut heatmap = self.empty_heatmap();
        for bomb in &map.bombs {
            let intensity = 1.0 / ((bomb.timer.max(1)) as f32);
            let row = bomb.position.row.get();
            let col = bomb.position.col.get();
            let idx = self.idx(row, col);
            heatmap[idx] = heatmap[idx].max(intensity);

            for &(dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                for distance in 1..=self.map_settings.bomb_radius {
                    let nr_isize = row as isize + dr * distance as isize;
                    let nc_isize = col as isize + dc * distance as isize;
                    if nr_isize < 0 || nc_isize < 0 {
                        break;
                    }
                    let nr = nr_isize as usize;
                    let nc = nc_isize as usize;

                    if self.out_of_bounds(nr, nc) {
                        break;
                    }

                    let cell = self.get_map_cell(nr, nc, map);
                    let idx = self.idx(nr, nc);
                    heatmap[idx] = heatmap[idx].max(intensity);

                    if cell == 'W' || cell == '.' {
                        break;
                    }
                }
            }
        }
        heatmap
    }

    fn find_escape_path(
        &self,
        map: &Map,
        start_row: usize,
        start_col: usize,
        danger_heatmap: &Vec<f32>,
    ) -> Option<(Command, usize)> {
        let mut visited = vec![false; danger_heatmap.len()];
        let mut queue = VecDeque::new();

        let start_idx = self.idx(start_row, start_col);
        if danger_heatmap[start_idx] == 0.0 {
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

            if danger_heatmap[idx] == 0.0 {
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

    fn propagate_heatmap(&self, map: &Map, heatmap: &Vec<f32>) -> Vec<f32> {
        let mut propagated = heatmap.clone();

        for row in 0..self.map_settings.size {
            for col in 0..self.map_settings.size {
                let index = self.idx(row, col);
                let original = heatmap[index];
                if original <= 0.0 {
                    continue;
                }

                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nr = (row as isize + dr) as isize;
                    let nc = (col as isize + dc) as isize;
                    if nr < 0 || nc < 0 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if self.out_of_bounds(nr, nc) || self.is_wall(map, nr, nc) {
                        continue;
                    }
                    let idx = self.idx(nr, nc);
                    propagated[idx] += original * 0.25;
                }
                propagated[index] += original * 0.25;
            }
        }

        propagated
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

    fn safe_neighbor_ratio(&self, map: &Map, row: usize, col: usize, danger_heatmap: &Vec<f32>) -> f32 {
        let mut total = 0;
        let mut safe = 0;
        for action in [Command::Up, Command::Down, Command::Left, Command::Right] {
            if let Some((nr, nc)) = self.apply_action(row, col, action) {
                if self.is_clear(map, nr, nc) {
                    total += 1;
                    if danger_heatmap[self.idx(nr, nc)] == 0.0 {
                        safe += 1;
                    }
                }
            }
        }
        if total == 0 {
            0.0
        } else {
            safe as f32 / total as f32
        }
    }

    fn center_bias(&self, row: usize, col: usize) -> f32 {
        let center = (self.map_settings.size as f32 - 1.0) / 2.0;
        let dist_row = (row as f32 - center).abs();
        let dist_col = (col as f32 - center).abs();
        let max_dist = center * 2.0;
        1.0 - ((dist_row + dist_col) / max_dist)
    }

    fn bomb_kill_score(&self, map: &Map, row: usize, col: usize) -> f32 {
    let mut best: f32 = 0.0;
        for player in map.get_alive_players() {
            if player.id == self.id {
                continue;
            }
            let prow = player.position.row.get();
            let pcol = player.position.col.get();
            if prow == row {
                let dist = pcol.abs_diff(col);
                if dist as usize <= self.map_settings.bomb_radius && self.line_clear(map, row, col, prow, pcol) {
                    let score = 1.0 - (dist as f32 / (self.map_settings.bomb_radius as f32 + 1.0));
                    best = best.max(score);
                }
            } else if pcol == col {
                let dist = prow.abs_diff(row);
                if dist as usize <= self.map_settings.bomb_radius && self.line_clear(map, row, col, prow, pcol) {
                    let score = 1.0 - (dist as f32 / (self.map_settings.bomb_radius as f32 + 1.0));
                    best = best.max(score);
                }
            }
        }
        best
    }

    fn adjacent_enemy_score(&self, map: &Map, row: usize, col: usize) -> f32 {
    let mut best: f32 = 0.0;
        for player in map.get_alive_players() {
            if player.id == self.id {
                continue;
            }
            let prow = player.position.row.get();
            let pcol = player.position.col.get();
            let manhattan = prow.abs_diff(row) + pcol.abs_diff(col);
            let score = (1.0 - (manhattan as f32 / 6.0)).max(0.0);
            best = best.max(score);
        }
        best
    }

    fn line_clear(&self, map: &Map, row: usize, col: usize, target_row: usize, target_col: usize) -> bool {
        if row == target_row {
            let (start, end) = if col < target_col { (col + 1, target_col) } else { (target_col + 1, col) };
            for c in start..end {
                let cell = self.get_map_cell(row, c, map);
                if cell == 'W' || cell == '.' {
                    return false;
                }
            }
        } else if col == target_col {
            let (start, end) = if row < target_row { (row + 1, target_row) } else { (target_row + 1, row) };
            for r in start..end {
                let cell = self.get_map_cell(r, col, map);
                if cell == 'W' || cell == '.' {
                    return false;
                }
            }
        }
        true
    }

    fn encode_action(&self, action: Command) -> [f32; ACTION_DIM] {
        let mut encoding = [0.0_f32; ACTION_DIM];
        let index = match action {
            Command::Up => 0,
            Command::Down => 1,
            Command::Left => 2,
            Command::Right => 3,
            Command::Wait => 4,
            Command::PlaceBomb => 5,
        };
        encoding[index] = 1.0;
        encoding
    }

    fn apply_action(&self, row: usize, col: usize, action: Command) -> Option<(usize, usize)> {
        match action {
            Command::Up => row.checked_sub(1).map(|r| (r, col)),
            Command::Down => {
                if row + 1 < self.map_settings.size {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            Command::Left => col.checked_sub(1).map(|c| (row, c)),
            Command::Right => {
                if col + 1 < self.map_settings.size {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
            Command::Wait | Command::PlaceBomb => Some((row, col)),
        }
    }

    fn empty_heatmap(&self) -> Vec<f32> {
        vec![0.0; self.map_settings.size * self.map_settings.size]
    }

    fn is_wall(&self, map: &Map, row: usize, col: usize) -> bool {
        let cell = self.get_map_cell(row, col, map);
        cell == 'W' || cell == '.'
    }

    fn is_clear(&self, map: &Map, row: usize, col: usize) -> bool {
        if let Some(shrink) = self.next_shrink_location {
            if shrink.row.get() == row && shrink.col.get() == col {
                return false;
            }
        }
        self.get_map_cell(row, col, map) == ' '
    }

    fn get_map_cell(&self, row: usize, col: usize, map: &Map) -> char {
        self.get_grid_value(&map.grid.tiles, row, col)
    }

    fn get_grid_value<T: Copy>(&self, grid: &Vec<T>, row: usize, col: usize) -> T {
        grid[self.idx(row, col)]
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.map_settings.size + col
    }

    fn out_of_bounds(&self, row: usize, col: usize) -> bool {
        row >= self.map_settings.size || col >= self.map_settings.size
    }

    fn get_correct_init_list(loc: Coord, height: i32, width: i32) -> Vec<Command> {
        // Start with the base script
        let mut script = NeuralBot::hardcoded_script();

        // Adjust for row
        if loc.row.get() >= height as usize / 2 {
            script = script
                .into_iter()
                .map(|c| match c {
                    Command::Up => Command::Down,
                    Command::Down => Command::Up,
                    other => other,
                })
                .collect();
        }

        // Adjust for column
        if loc.col.get() >= width as usize / 2 {
            script = script
                .into_iter()
                .map(|c| match c {
                    Command::Left => Command::Right,
                    Command::Right => Command::Left,
                    other => other,
                })
                .collect();
        }

        script
    }
}

impl Bot for NeuralBot {
    fn start_game(&mut self, settings: &MapConfig, bot_name: String, bot_id: usize) -> bool {
        self.id = bot_id;
        self.name = bot_name;
        self.map_settings = settings.clone();
        self.turn = 0;
        self.next_shrink_location = None;
        self.last_debug.clear();
        true
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        if !self.initialized {
            let height = self.map_settings.size as i32;
            let width = self.map_settings.size as i32;

            self.command_list = NeuralBot::get_correct_init_list(player_location, height, width);

            self.initialized = true;
            self.current_index = 0;
            self.looping = false;
        }

        //max size game 19x19 = 187x17
        // Run initial script
        if !self.looping {
            if self.current_index < self.command_list.len() 
            && (self.map_settings.size != 7 || self.current_index < 2)
            && (self.map_settings.size != 9 || self.current_index < 18) 
            && (self.map_settings.size != 11 || self.current_index < 18)
            && (self.map_settings.size != 13 || self.current_index < 27)
            && (self.map_settings.size != 15 || self.current_index < 51)
            {
                let cmd = self.command_list[self.current_index];
                self.current_index += 1;
                return cmd;
            }
            // Switch to rotation mode
            self.looping = true;
            self.current_index = 0;
        }

        if map.map_settings.endgame <= self.turn {
            self.next_shrink_location = calculate_shrink_location(
                self.turn - map.map_settings.endgame,
                map.map_settings.size,
            );
        } else {
            self.next_shrink_location = None;
        }

        self.turn += 1;
        self.decide_move(map, player_location)
    }

    fn get_debug_info(&self) -> String {
        //self.last_debug.clone()
        String::new()
    }
}
