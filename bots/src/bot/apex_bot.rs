//! ApexBot - a simulation driven bomberman bot.
//!
//! The bot never guesses. Every turn it rebuilds an exact model of the near
//! future - bomb timers, chain reactions, blast shapes, blocks that are about
//! to be destroyed and the walls the endgame shrink is going to drop - and then
//! answers two questions for every legal command:
//!
//! 1. how many turns can I *guarantee* to stay alive after this command?
//! 2. how much does this command shorten the opponent's guaranteed life?
//!
//! Survival is always the primary key, so the bot only ever commits to a bomb
//! when a guaranteed escape route exists. Aggression only breaks ties.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::grid::cell::CellType;
use game::map::map::Map;
use game::map::shrink::calculate_shrink_location;
use game::map::structs::map_config::MapConfig;

/// Sentinel for "never" / "unreachable".
const NEVER: usize = usize::MAX;
/// Minimum number of turns the survival search looks ahead.
const MIN_HORIZON: usize = 8;
/// Longest look ahead used once the map starts collapsing.
const ENDGAME_HORIZON: usize = 110;
/// Travel cost of a tile that first has to be bombed away.
const BLOCK_COST: usize = 7;
/// Number of refinement passes for the chain reaction fixpoint.
const CHAIN_PASSES: usize = 6;
/// Scoring weights. These were tuned by running the bot through the full
/// gauntlet: every opponent, every map size, 2/3/4 players, every seat.
mod weight {
    /// Being able to survive an opponent bombing on the spot.
    pub const EXPOSED: f64 = 350.0;
    /// Guaranteed survival over the next few turns - effectively absolute.
    pub const SHORT_LIFE: f64 = 3000.0;
    /// Guaranteed survival beyond that, i.e. outlasting the shrink.
    pub const LONG_LIFE: f64 = 40.0;
    /// Size of the patch of floor the bot can move around in.
    pub const ROOM: f64 = 15.0;
    /// Blocks a bomb clears away.
    pub const RUBBLE: f64 = 12.0;
    /// Closing in on the opponent being hunted.
    pub const APPROACH: f64 = 8.0;
    /// Shrinking the area an opponent can still reach.
    pub const SQUEEZE: f64 = 30.0;
    /// Shortening the guaranteed life of an opponent.
    pub const PRESS: f64 = 250.0;
    /// A bomb whose blast covers an opponent.
    pub const SCARE: f64 = 80.0;
    /// Heading for the tile the shrink walls last: base and endgame ramp.
    pub const REFUGE: f64 = 1.5;
    pub const REFUGE_RAMP: f64 = 12.0;
    /// A bomb that opens the road to the opponent, or to the refuge.
    pub const BREACH_ENEMY: f64 = 25.0;
    pub const BREACH_REFUGE: f64 = 10.0;
    pub const BREACH_REFUGE_RAMP: f64 = 90.0;
    /// Neighbouring tiles that stay usable.
    pub const MOBILITY: f64 = 1.5;
    /// Loitering inside a blast that is already ticking.
    pub const LINGER: f64 = 15.0;
    /// Dropping a bomb for no particular reason.
    pub const BOMB: f64 = 3.0;
}

/// Turns before the endgame at which the bot starts heading for the middle.
const ENDGAME_RUNWAY: f64 = 600.0;
/// Most opponents whose hypothetical bombs are modelled at once.
const MAX_THREATS: usize = 2;
/// Extra Manhattan distance beyond the blast radius at which an opponent is
/// still treated as able to bomb us.
const THREAT_SPAN: usize = 3;

/// Everything the bot needs to know about the next `horizon` turns.
///
/// All times are *arrival times*: `a` is the tile the bot occupies after its
/// move in turn `now + a - 1`, which is exactly the tile that the bomb- and
/// shrink-processing at the end of that turn will judge.
struct Field {
    /// First arrival time at which the tile may be entered (`NEVER` = solid).
    open_from: Vec<usize>,
    /// A bomb sits on the tile up to and including this arrival time.
    bomb_until: Vec<usize>,
    /// Bitmask of arrival times at which standing on the tile is lethal.
    blast: Vec<u64>,
    /// From this arrival time on the tile has been walled by the shrink.
    shrink_from: Vec<usize>,
    /// Destroyable tiles removed by the bomb this field adds.
    fresh_kills: Vec<usize>,
    horizon: usize,
}

impl Field {
    /// May the bot stand on `cell` at arrival time `a` and live?
    ///
    /// `start` is the tile the bot occupies right now; it is allowed to stay
    /// there for one more turn even after dropping a bomb under itself.
    #[inline]
    fn allowed(&self, cell: usize, a: usize, start: usize) -> bool {
        if a < self.open_from[cell] {
            return false;
        }
        if a >= self.shrink_from[cell] {
            return false;
        }
        if a <= self.bomb_until[cell] && !(cell == start && a == 1) {
            return false;
        }
        if a < 64 && (self.blast[cell] >> a) & 1 != 0 {
            return false;
        }
        true
    }
}

/// A command together with the tile the bot ends up on.
struct Candidate {
    command: Command,
    target: usize,
    places_bomb: bool,
}

pub struct ApexBot {
    name: String,
    id: usize,
    turn: usize,
    size: usize,
    endgame: usize,
    /// For every tile: the shrink sequence number that turns it into a wall.
    shrink_order: Vec<usize>,
    neighbours: Vec<Vec<usize>>,
    /// The tile the shrink walls last - the only place worth being at the end.
    refuge: usize,
    last_cell: usize,
    stuck: usize,
    rng: u64,
    debug: String,
}

impl Default for ApexBot {
    fn default() -> Self {
        Self::new()
    }
}

impl ApexBot {
    pub fn new() -> Self {
        ApexBot {
            name: "ApexBot".to_string(),
            id: 0,
            turn: 0,
            size: 0,
            endgame: usize::MAX,
            shrink_order: Vec::new(),
            neighbours: Vec::new(),
            refuge: 0,
            last_cell: NEVER,
            stuck: 0,
            rng: 0x9E37_79B9_7F4A_7C15,
            debug: String::new(),
        }
    }

    #[inline]
    fn index(&self, coord: Coord) -> usize {
        coord.row.get() * self.size + coord.col.get()
    }

    /// Build the lookup tables that only depend on the map size.
    fn prepare(&mut self, size: usize) {
        if self.size == size && !self.neighbours.is_empty() {
            return;
        }
        self.size = size;
        let cells = size * size;

        self.shrink_order = vec![NEVER; cells];
        for step in 0..(size - 2) * (size - 2) {
            let Some(coord) = calculate_shrink_location(step, size) else {
                break;
            };
            let cell = self.index(coord);
            if cell < cells && self.shrink_order[cell] == NEVER {
                self.shrink_order[cell] = step;
            }
        }

        self.refuge = (0..cells)
            .filter(|&cell| self.shrink_order[cell] != NEVER)
            .max_by_key(|&cell| self.shrink_order[cell])
            .unwrap_or(0);

        self.neighbours = vec![Vec::with_capacity(4); cells];
        for row in 0..size {
            for col in 0..size {
                let cell = row * size + col;
                if row > 0 {
                    self.neighbours[cell].push(cell - size);
                }
                if row + 1 < size {
                    self.neighbours[cell].push(cell + size);
                }
                if col > 0 {
                    self.neighbours[cell].push(cell - 1);
                }
                if col + 1 < size {
                    self.neighbours[cell].push(cell + 1);
                }
            }
        }
    }

    /// Manhattan distance between two tiles.
    #[inline]
    fn walk_span(&self, from: usize, to: usize) -> usize {
        let (from_row, from_col) = (from / self.size, from % self.size);
        let (to_row, to_col) = (to / self.size, to % self.size);
        from_row.abs_diff(to_row) + from_col.abs_diff(to_col)
    }

    #[inline]
    fn step(&self, cell: usize, direction: usize) -> Option<usize> {
        let row = cell / self.size;
        let col = cell % self.size;
        match direction {
            0 if row > 0 => Some(cell - self.size),
            1 if row + 1 < self.size => Some(cell + self.size),
            2 if col > 0 => Some(cell - 1),
            3 if col + 1 < self.size => Some(cell + 1),
            _ => None,
        }
    }

    /// Model the coming turns. `extra_bombs` are bombs assumed to be dropped
    /// this very turn - the bot's own bomb and/or the bombs an opponent could
    /// drop under itself right now. `own` marks which of them is the bot's, so
    /// the blocks it clears can be credited to the command being scored.
    fn build_field(
        &self,
        map: &Map,
        me: usize,
        turn: usize,
        horizon: usize,
        extra_bombs: &[usize],
        own: Option<usize>,
        ghosts: &[usize],
    ) -> Field {
        let cells = self.size * self.size;
        let radius = map.map_settings.bomb_radius;
        let fuse = map.map_settings.bomb_timer.max(1);

        let mut open_from = vec![0usize; cells];
        let mut destroyable = vec![false; cells];
        let mut solid = vec![false; cells];
        for cell in 0..cells {
            match map.grid.tiles[cell] {
                'W' => {
                    open_from[cell] = NEVER;
                    solid[cell] = true;
                }
                '.' => {
                    open_from[cell] = NEVER;
                    destroyable[cell] = true;
                }
                // A living player physically blocks the tile. They may well
                // step aside, but an escape route is only worth anything when
                // it does not depend on somebody else's good will.
                'P' => open_from[cell] = NEVER,
                _ => {}
            }
        }
        // Wherever the bot stands right now is by definition reachable.
        open_from[me] = 0;

        // Every bomb with the arrival time at which it detonates.
        let mut positions: Vec<usize> = Vec::with_capacity(map.bombs.len() + 1);
        let mut fuses: Vec<usize> = Vec::with_capacity(map.bombs.len() + 1);
        for bomb in &map.bombs {
            positions.push(self.index(bomb.position));
            fuses.push(bomb.timer.max(1));
        }
        let mut fresh = None;
        for &cell in extra_bombs {
            if positions.contains(&cell) {
                continue;
            }
            if own == Some(cell) {
                fresh = Some(positions.len());
            }
            positions.push(cell);
            fuses.push(fuse);
        }

        // Chain reactions: a bomb caught in a blast detonates on the spot, so
        // the real fuse of a bomb is the earliest blast that reaches it. Blast
        // shapes in turn depend on which blocks are already gone. Both are
        // resolved together with a small fixpoint.
        let mut blast = vec![0u64; cells];
        let mut cleared = vec![(NEVER, NEVER); cells]; // (time, bomb) first to clear
        let mut cleared_again = vec![NEVER; cells]; // earliest time by a second bomb
        let mut fresh_kills = Vec::new();

        for pass in 0..CHAIN_PASSES {
            let was_cleared = cleared.clone();
            let was_cleared_again = cleared_again.clone();
            let previous_fuses = fuses.clone();

            blast.iter_mut().for_each(|mask| *mask = 0);
            cleared.iter_mut().for_each(|slot| *slot = (NEVER, NEVER));
            cleared_again.iter_mut().for_each(|slot| *slot = NEVER);
            fresh_kills.clear();

            for bomb in 0..positions.len() {
                let origin = positions[bomb];
                let time = fuses[bomb];
                mark_blast(&mut blast, origin, time);
                for direction in 0..4 {
                    let mut cell = origin;
                    for _ in 0..radius {
                        let Some(next) = self.step(cell, direction) else {
                            break;
                        };
                        cell = next;
                        if solid[cell] {
                            break;
                        }
                        mark_blast(&mut blast, cell, time);
                        // Anything standing here detonates along with us.
                        for other in 0..positions.len() {
                            if positions[other] == cell && fuses[other] > time {
                                fuses[other] = time;
                            }
                        }
                        if !destroyable[cell] {
                            continue;
                        }
                        if Some(bomb) == fresh {
                            fresh_kills.push(cell);
                        }
                        record_clear(&mut cleared, &mut cleared_again, cell, time, bomb);
                        let gone = (was_cleared[cell].0 <= time && was_cleared[cell].1 != bomb)
                            || was_cleared_again[cell] <= time;
                        if !gone {
                            break;
                        }
                    }
                }
            }

            if pass > 0
                && cleared == was_cleared
                && cleared_again == was_cleared_again
                && fuses == previous_fuses
            {
                break;
            }
        }

        let mut bomb_until = vec![0usize; cells];
        for bomb in 0..positions.len() {
            let tile = positions[bomb];
            bomb_until[tile] = bomb_until[tile].max(fuses[bomb]);
        }

        // A destroyed block becomes walkable one turn after it blew up.
        for cell in 0..cells {
            if destroyable[cell] && cleared[cell].0 != NEVER {
                open_from[cell] = cleared[cell].0 + 1;
            }
        }

        // The shrink walls tile `i` at the end of turn `endgame + i`.
        let mut shrink_from = vec![NEVER; cells];
        for cell in 0..cells {
            let order = self.shrink_order[cell];
            if order != NEVER {
                shrink_from[cell] = (self.endgame + order + 1).saturating_sub(turn);
            }
        }

        // Opponents are bodies, not scenery: a tile an opponent can stand on
        // before the bot gets there is not an escape route, it is a trap.
        if !ghosts.is_empty() {
            let mut reach = vec![NEVER; cells];
            let mut frontier: Vec<usize> = Vec::new();
            for &ghost in ghosts {
                if reach[ghost] == NEVER {
                    reach[ghost] = 0;
                    frontier.push(ghost);
                }
            }
            let mut distance = 0;
            while !frontier.is_empty() {
                distance += 1;
                let mut next_frontier = Vec::new();
                for &cell in &frontier {
                    for &step in &self.neighbours[cell] {
                        if reach[step] != NEVER || solid[step] || destroyable[step] {
                            continue;
                        }
                        reach[step] = distance;
                        next_frontier.push(step);
                    }
                }
                frontier = next_frontier;
            }
            for cell in 0..cells {
                if reach[cell] != NEVER {
                    shrink_from[cell] = shrink_from[cell].min(reach[cell] + 1);
                }
            }
        }

        fresh_kills.sort_unstable();
        fresh_kills.dedup();

        Field {
            open_from,
            bomb_until,
            blast,
            shrink_from,
            fresh_kills,
            horizon,
        }
    }

    /// `table[a * cells + tile]` = how many further turns can be survived after
    /// arriving on `tile` at arrival time `a`. Solved backwards from the horizon.
    fn survival_table(&self, field: &Field, start: usize) -> Vec<u16> {
        let cells = self.size * self.size;
        let horizon = field.horizon;
        let mut table = vec![0u16; (horizon + 1) * cells];

        for time in (1..horizon).rev() {
            let (head, tail) = table.split_at_mut((time + 1) * cells);
            let current = &mut head[time * cells..];
            let later = &tail[..cells];
            for cell in 0..cells {
                if !field.allowed(cell, time, start) {
                    continue;
                }
                let mut best = 0;
                if field.allowed(cell, time + 1, start) {
                    best = later[cell] + 1;
                }
                for &next in &self.neighbours[cell] {
                    if field.allowed(next, time + 1, start) {
                        best = best.max(later[next] + 1);
                    }
                }
                current[cell] = best;
            }
        }

        table
    }

    /// Turns of life that can be guaranteed after ending a turn on `cell`.
    #[inline]
    fn depth_at(&self, field: &Field, table: &[u16], cell: usize, start: usize) -> usize {
        if field.allowed(cell, 1, start) {
            1 + table[self.size * self.size + cell] as usize
        } else {
            0
        }
    }

    /// Turns of life a player on `from` can guarantee when they play perfectly.
    /// `blocked` is a tile they cannot use (the tile this bot occupies).
    fn best_depth_from(
        &self,
        field: &Field,
        table: &[u16],
        from: usize,
        start: usize,
        blocked: usize,
    ) -> usize {
        let mut best = self.depth_at(field, table, from, start);
        for &next in &self.neighbours[from] {
            if next != blocked {
                best = best.max(self.depth_at(field, table, next, start));
            }
        }
        best
    }

    /// How many different tiles a player on `from` can still reach without
    /// giving up their life within `reach` turns. This is the pressure metric:
    /// a bomb that cuts an opponent's corridor collapses their free area long
    /// before it becomes an outright kill.
    fn free_area(
        &self,
        field: &Field,
        table: &[u16],
        from: usize,
        start: usize,
        blocked: usize,
        reach: usize,
    ) -> usize {
        let cells = self.size * self.size;
        let mut seen = vec![false; cells];
        let mut visited = vec![false; (reach + 1) * cells];
        let mut layer = vec![from];
        let mut area = 0;

        for time in 1..=reach {
            let mut next_layer = Vec::new();
            for &cell in &layer {
                for &step in self.neighbours[cell].iter().chain(std::iter::once(&cell)) {
                    if step == blocked || visited[time * cells + step] {
                        continue;
                    }
                    if !field.allowed(step, time, start) {
                        continue;
                    }
                    if time + (table[time * cells + step] as usize) < reach {
                        continue;
                    }
                    visited[time * cells + step] = true;
                    next_layer.push(step);
                    if !seen[step] {
                        seen[step] = true;
                        area += 1;
                    }
                }
            }
            layer = next_layer;
        }
        area
    }

    /// Size of the connected patch of floor around `from`, counting tiles that
    /// a bomb is about to open up. Being locked in a pocket is how bots die.
    fn room(&self, field: &Field, from: usize) -> usize {
        let cells = self.size * self.size;
        if field.open_from[from] == NEVER {
            return 0;
        }
        let mut seen = vec![false; cells];
        let mut stack = vec![from];
        seen[from] = true;
        let mut count = 0;
        while let Some(cell) = stack.pop() {
            count += 1;
            for &next in &self.neighbours[cell] {
                if !seen[next] && field.open_from[next] != NEVER {
                    seen[next] = true;
                    stack.push(next);
                }
            }
        }
        count
    }

    /// Weighted distance from `from` to every tile; destroyable tiles cost more
    /// because they have to be bombed away first.
    fn distance_field(&self, map: &Map, from: usize) -> Vec<usize> {
        let cells = self.size * self.size;
        let mut distance = vec![NEVER; cells];
        let mut queue = BinaryHeap::new();
        distance[from] = 0;
        queue.push(Reverse((0usize, from)));

        while let Some(Reverse((cost, cell))) = queue.pop() {
            if cost > distance[cell] {
                continue;
            }
            for &next in &self.neighbours[cell] {
                let step = match map.grid.tiles[next] {
                    'W' => continue,
                    '.' => BLOCK_COST,
                    'P' => BLOCK_COST,
                    'B' => 2,
                    _ => 1,
                };
                if cost + step < distance[next] {
                    distance[next] = cost + step;
                    queue.push(Reverse((cost + step, next)));
                }
            }
        }
        distance
    }

    fn jitter(&mut self) -> f64 {
        self.rng = self
            .rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.rng >> 40) as f64 / (1u64 << 24) as f64) - 0.5
    }

    fn decide(&mut self, map: &Map, me: Coord) -> Command {
        let turn = self.turn;
        let my_cell = self.index(me);
        let fuse = map.map_settings.bomb_timer.max(1);

        // Two look aheads: a short one that is enough to judge bombs, and a
        // long one that reaches into the collapsing map so that the survival
        // search itself - not a heuristic - decides where to be.
        let mut tactical = MIN_HORIZON.max(fuse + 2);
        for bomb in &map.bombs {
            tactical = tactical.max(bomb.timer + 2);
        }
        let inner = (self.size - 2) * (self.size - 2);
        let endgame_near = turn + ENDGAME_HORIZON >= self.endgame;
        let horizon = if endgame_near {
            let walled = (turn + 1).saturating_sub(self.endgame);
            tactical.max((inner - walled.min(inner) + 2).min(ENDGAME_HORIZON))
        } else {
            tactical
        };

        let enemies: Vec<usize> = map
            .players
            .iter()
            .filter(|player| player.id != self.id && player.is_alive())
            .map(|player| self.index(player.position))
            .filter(|&cell| cell != my_cell && cell < self.size * self.size)
            .collect();

        // Hunt the opponent we can actually reach first.
        let mut distance = vec![NEVER; self.size * self.size];
        let mut target = None;
        let mut target_cost = NEVER;
        for &enemy in &enemies {
            let field = self.distance_field(map, enemy);
            let cost = field[my_cell];
            if target.is_none() || cost < target_cost {
                target_cost = cost;
                target = Some(enemy);
                distance = field;
            }
        }

        // Opponents close enough to drop a bomb that could still reach us. The
        // bot refuses to stand anywhere those bombs would be inescapable.
        let mut threats: Vec<usize> = enemies
            .iter()
            .copied()
            .filter(|&enemy| {
                self.walk_span(enemy, my_cell) <= map.map_settings.bomb_radius + THREAT_SPAN
            })
            .collect();
        threats.sort_by_key(|&enemy| self.walk_span(enemy, my_cell));
        threats.truncate(MAX_THREATS);

        let reach = tactical.min(fuse + 4);
        // As the endgame approaches, getting to the last tile the shrink will
        // wall matters more than anything the opponents are doing.
        let refuge_distance = self.distance_field(map, self.refuge);
        let urgency = (((turn as f64 + ENDGAME_RUNWAY) - self.endgame as f64) / ENDGAME_RUNWAY)
            .clamp(0.0, 1.0);
        let refuge_pull = weight::REFUGE + weight::REFUGE_RAMP * urgency;

        let quiet = self.build_field(map, my_cell, turn, horizon, &[], None, &[]);
        let quiet_survival = self.survival_table(&quiet, my_cell);
        let quiet_enemy_depth: Vec<usize> = enemies
            .iter()
            .map(|&enemy| {
                self.best_depth_from(&quiet, &quiet_survival, enemy, my_cell, my_cell)
            })
            .collect();
        let quiet_target_depth = target
            .map(|enemy| self.best_depth_from(&quiet, &quiet_survival, enemy, my_cell, my_cell))
            .unwrap_or(0);
        let quiet_target_area = target
            .map(|enemy| self.free_area(&quiet, &quiet_survival, enemy, my_cell, my_cell, reach))
            .unwrap_or(0);

        let armed = (map.grid.cell_type(me) != CellType::Bomb).then(|| {
            let field = self.build_field(map, my_cell, turn, horizon, &[my_cell], Some(my_cell), &[]);
            let survival = self.survival_table(&field, my_cell);
            (field, survival)
        });

        // The same two worlds, but with every nearby opponent bombing at once.
        let menace = (!threats.is_empty()).then(|| {
            let field = self.build_field(map, my_cell, turn, tactical, &threats, None, &threats);
            let survival = self.survival_table(&field, my_cell);
            (field, survival)
        });
        let armed_menace = menace.as_ref().and(armed.as_ref()).map(|_| {
            let mut bombs = threats.clone();
            bombs.push(my_cell);
            let field = self.build_field(map, my_cell, turn, tactical, &bombs, Some(my_cell), &threats);
            let survival = self.survival_table(&field, my_cell);
            (field, survival)
        });

        let mut candidates = Vec::with_capacity(6);
        for (command, destination) in [
            (Command::Up, me.move_up()),
            (Command::Down, me.move_down()),
            (Command::Left, me.move_left()),
            (Command::Right, me.move_right()),
        ] {
            if let Some(coord) = destination
                && map.grid.can_move_to(coord)
            {
                candidates.push(Candidate {
                    command,
                    target: self.index(coord),
                    places_bomb: false,
                });
            }
        }
        candidates.push(Candidate {
            command: Command::Wait,
            target: my_cell,
            places_bomb: false,
        });
        if armed.is_some() {
            candidates.push(Candidate {
                command: Command::PlaceBomb,
                target: my_cell,
                places_bomb: true,
            });
        }

        let mut best_command = Command::Wait;
        let mut best_key = (false, 0usize, f64::NEG_INFINITY);

        for candidate in &candidates {
            let (field, survival) = match (candidate.places_bomb, armed.as_ref()) {
                (true, Some((field, survival))) => (field, survival),
                _ => (&quiet, &quiet_survival),
            };

            let depth = self.depth_at(field, survival, candidate.target, my_cell);

            // Survivability if every nearby opponent bombs on the spot.
            // The bot's own bomb is already covered by `depth`; counting it a
            // second time here makes every bomb look twice as dangerous.
            let counter = if candidate.places_bomb {
                armed_menace.as_ref()
            } else {
                menace.as_ref()
            };
            let exposed = match counter {
                Some((field, survival)) => {
                    self.depth_at(field, survival, candidate.target, my_cell)
                }
                None => tactical,
            };

            // Does this command finish the game right now?
            let wins_now = !enemies.is_empty()
                && depth > 0
                && enemies.iter().enumerate().all(|(slot, &enemy)| {
                    let life = if candidate.places_bomb {
                        self.best_depth_from(field, survival, enemy, my_cell, my_cell)
                    } else {
                        quiet_enemy_depth[slot]
                    };
                    life == 0
                });

            let mut score = 0.0;

            // Staying alive over the next few turns is close to absolute;
            // outliving the shrink is merely very valuable, so that it can be
            // traded for a kill when that is the shorter road to the win.
            score += weight::SHORT_LIFE * depth.min(tactical) as f64;
            score += weight::LONG_LIFE * depth.saturating_sub(tactical) as f64;

            if let Some(enemy) = target {
                let (pressed_depth, pressed_area) = if candidate.places_bomb {
                    (
                        self.best_depth_from(field, survival, enemy, my_cell, my_cell),
                        self.free_area(field, survival, enemy, my_cell, my_cell, reach),
                    )
                } else {
                    (quiet_target_depth, quiet_target_area)
                };

                // Cornering the opponent is worth a lot; killing outright wins.
                score += weight::SQUEEZE * (quiet_target_area as f64 - pressed_area as f64);
                score += weight::PRESS * (quiet_target_depth as f64 - pressed_depth as f64);
                if pressed_depth <= fuse && quiet_target_depth > fuse && depth > tactical {
                    score += 50_000.0;
                }
                if candidate.places_bomb && enemies.iter().any(|&cell| field.blast[cell] != 0) {
                    score += weight::SCARE;
                }
                score -= weight::APPROACH * distance[candidate.target].min(500) as f64;
            }

            // Standing somewhere an opponent could bomb us to death is what
            // separates a dead bot from a live one.
            // Once the walls are closing in, refusing every risk is itself
            // the losing move: the bot has to bomb its way somewhere better.
            score += weight::EXPOSED * exposed.min(reach) as f64;
            score += weight::ROOM * self.room(field, candidate.target).min(40) as f64;

            score -= refuge_pull * refuge_distance[candidate.target].min(500) as f64;

            // Do not linger inside a blast that is already ticking.
            if quiet.blast[candidate.target] != 0 {
                score -= weight::LINGER;
            }

            // Keep the options open.
            let mobility = self.neighbours[candidate.target]
                .iter()
                .filter(|&&next| field.allowed(next, 2, my_cell))
                .count();
            score += weight::MOBILITY * mobility as f64;

            // Stay where the collapsing map keeps a floor the longest.
            if endgame_near {
                score += 3.0 * field.shrink_from[candidate.target].min(200) as f64;
            }

            if candidate.places_bomb {
                score -= weight::BOMB;
                score += weight::RUBBLE * field.fresh_kills.len() as f64;
                // Blowing open the way to whatever we are heading for.
                let to_enemy = distance[my_cell];
                let to_refuge = refuge_distance[my_cell];
                if field.fresh_kills.iter().any(|&cell| distance[cell] < to_enemy) {
                    score += weight::BREACH_ENEMY;
                }
                if field
                    .fresh_kills
                    .iter()
                    .any(|&cell| refuge_distance[cell] < to_refuge)
                {
                    score += weight::BREACH_REFUGE + weight::BREACH_REFUGE_RAMP * urgency;
                }
            }

            if self.stuck > 6 {
                score += self.jitter() * 4.0 * self.stuck as f64;
            }

            // Surviving for certain outranks everything else; the rest is
            // weighed against each other.
            // Not dying over the next few turns is non negotiable. Everything
            // beyond that - outlasting the shrink, hunting, opening up the map
            // - is weighed against the rest in the score.
            let key = (wins_now, depth.min(tactical), score);
            if (key.0, key.1) > (best_key.0, best_key.1)
                || ((key.0, key.1) == (best_key.0, best_key.1) && key.2 > best_key.2)
            {
                best_key = key;
                best_command = candidate.command;
            }
        }

        self.debug = format!(
            "t{turn} safe {}/{tactical} refuge {} score {:.0}",
            best_key.1,
            refuge_distance[my_cell],
            best_key.2
        );
        best_command
    }
}

#[inline]
fn mark_blast(blast: &mut [u64], cell: usize, time: usize) {
    if time < 64 {
        blast[cell] |= 1u64 << time;
    }
}

/// Remember the two earliest (distinct bomb) times at which a block is cleared.
#[inline]
fn record_clear(
    first: &mut [(usize, usize)],
    second: &mut [usize],
    cell: usize,
    time: usize,
    bomb: usize,
) {
    if time < first[cell].0 {
        if first[cell].1 != bomb && first[cell].0 < second[cell] {
            second[cell] = first[cell].0;
        }
        first[cell] = (time, bomb);
    } else if first[cell].1 != bomb && time < second[cell] {
        second[cell] = time;
    }
}

impl Bot for ApexBot {
    fn start_game(&mut self, settings: &MapConfig, bot_name: String, bot_id: usize) -> bool {
        self.name = bot_name;
        self.id = bot_id;
        self.endgame = settings.endgame;
        self.turn = 0;
        self.last_cell = NEVER;
        self.stuck = 0;
        self.prepare(settings.size);
        true
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        self.prepare(map.map_settings.size);
        self.endgame = map.map_settings.endgame;

        let cell = self.index(player_location);
        if cell == self.last_cell {
            self.stuck += 1;
        } else {
            self.stuck = 0;
        }
        self.last_cell = cell;

        let command = self.decide(map, player_location);
        self.turn += 1;
        command
    }

    fn get_debug_info(&self) -> String {
        self.debug.clone()
    }
}
