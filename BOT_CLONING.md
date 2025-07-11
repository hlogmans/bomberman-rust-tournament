# Bot Cloning Feature

## Overview

The bot cloning feature allows you to create multiple instances of the same bot type. This is essential for running tournaments, parallel games, or any scenario where you need separate bot states.

## Why Bot Cloning?

In the original implementation, bots were trait objects (`Box<dyn Bot>`) that couldn't be copied using Rust's standard `Copy` or `Clone` traits. This made it difficult to:

- Run multiple games with the same bot configuration
- Create tournament systems where bots compete multiple times
- Save and restore bot states
- Run parallel games with separate bot instances

## Implementation

### The `clone_bot` Method

Every bot must implement the `clone_bot` method as part of the `Bot` trait:

```rust
pub trait Bot {
    fn name(&self) -> String;
    fn start_game(&mut self, map_settings: &MapSettings, bot_id: usize) -> bool;
    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command;
    
    /// Creates a clone of this bot as a new boxed trait object
    fn clone_bot(&self) -> Box<dyn Bot>;
}
```

### The `clone_bot` Helper Function

A convenient helper function is provided for cloning bots:

```rust
/// Creates a clone of a bot as a new boxed trait object
pub fn clone_bot(bot: &dyn Bot) -> Box<dyn Bot> {
    bot.clone_bot()
}
```

### Bot Implementation Requirements

For a bot to be cloneable, it must:

1. Implement the `Clone` trait (using `#[derive(Clone)]`)
2. Implement the `clone_bot` method in the `Bot` trait

Example implementation:

```rust
#[derive(Clone)]
pub struct MyBot {
    pub name: String,
    pub id: usize,
    // other fields...
}

impl Bot for MyBot {
    // ... other methods ...
    
    fn clone_bot(&self) -> Box<dyn Bot> {
        Box::new(self.clone())
    }
}
```

## Usage Examples

### Basic Cloning

```rust
use rust_bomberman::bot::{available_bots, clone_bot};

let bot_constructors = available_bots();
let original_bot = bot_constructors[0]("MyBot");
let cloned_bot = clone_bot(original_bot.as_ref());

// Both bots have the same configuration but are separate instances
assert_eq!(original_bot.name(), cloned_bot.name());
```

### Tournament Setup

```rust
use rust_bomberman::bot::{available_bots, clone_bot};
use rust_bomberman::game::Game;

let bot_constructors = available_bots();
let template_bot1 = bot_constructors[0]("Player1");
let template_bot2 = bot_constructors[1]("Player2");

// Create multiple games with cloned bots
for game_num in 0..10 {
    let game_bots = vec![
        clone_bot(template_bot1.as_ref()),
        clone_bot(template_bot2.as_ref()),
    ];
    
    let mut game = Game::build(11, 11, game_bots);
    let result = game.run();
    println!("Game {}: Winner is {:?}", game_num, result.winner);
}
```

### Parallel Games

```rust
use std::thread;
use rust_bomberman::bot::{available_bots, clone_bot};
use rust_bomberman::game::Game;

let bot_constructors = available_bots();
let template_bot1 = bot_constructors[0]("Player1");
let template_bot2 = bot_constructors[1]("Player2");

let mut handles = vec![];

// Run games in parallel threads
for game_id in 0..4 {
    let bot1 = clone_bot(template_bot1.as_ref());
    let bot2 = clone_bot(template_bot2.as_ref());
    
    let handle = thread::spawn(move || {
        let mut game = Game::build(11, 11, vec![bot1, bot2]);
        let result = game.run();
        (game_id, result)
    });
    
    handles.push(handle);
}

// Collect results
for handle in handles {
    let (game_id, result) = handle.join().unwrap();
    println!("Parallel game {}: Winner is {:?}", game_id, result.winner);
}
```

## Technical Details

### Memory Management

- Each cloned bot is a completely separate instance with its own memory
- Cloning creates a deep copy of all bot state
- The original bot and cloned bots are independent

### Performance Considerations

- Cloning is relatively lightweight for simple bots
- Complex bots with large internal state may have higher cloning costs
- Consider the trade-off between cloning cost and memory usage

### Thread Safety

- Cloned bots are separate instances and can be used in different threads
- No shared state between original and cloned bots
- Perfect for parallel execution scenarios

## Migration Guide

If you have existing bot implementations, here's how to add cloning support:

1. Add `#[derive(Clone)]` to your bot struct
2. Ensure all fields in your bot struct implement `Clone`
3. Add the `clone_bot` method to your `Bot` implementation:

```rust
fn clone_bot(&self) -> Box<dyn Bot> {
    Box::new(self.clone())
}
```

## Testing

The cloning feature includes comprehensive tests:

- `test_bot_cloning`: Tests basic cloning functionality
- `test_easy_bot_cloning`: Tests cloning of complex bots
- Integration tests verify cloned bots work in actual games

Run tests with:
```bash
cargo test bot::tests
```

## Example Program

See `examples/bot_cloning.rs` for a complete example demonstrating bot cloning:

```bash
cargo run --example bot_cloning
```

This example shows:
- Creating and cloning bots
- Verifying separate instances
- Setting up multiple games
- Practical usage patterns