use bots::random_bot::RandomBot;
use game::{game::game::Game, map::structs::map_config::MapConfig};

#[test]
fn integration_game_runs_and_has_winner() {
    // Arrange: maak twee bots aan
    let bot1 = Box::new(RandomBot::new());
    let bot2 = Box::new(RandomBot::new());

    // Start een spel met een kleine map zodat het snel klaar is
    let settings = MapConfig {
            bomb_timer: 4,
            bomb_radius: 3,
            endgame: 500,
            size: 7
        };
    let mut game = Game::build(vec![bot1, bot2], settings, None);

    // Act: speel maximaal 100 rondes of tot er een winnaar is
    let mut rounds = 0;
    while !game.map.has_winner() && rounds < 1000 {
        game.run_round( None);
        rounds += 1;
    }

    // Assert: er moet een winnaar zijn
    assert!(
        game.map.has_winner(),
        "Het spel zou binnen 100 rondes een winnaar moeten hebben"
    );
    let winner_name = game.winner_name();
    assert_ne!(
        winner_name, 
        "No winner yet",
        "Er zou een winnaar-naam moeten zijn als het spel klaar is"
    );
    println!("Winnaar: {:?}", winner_name);
}

#[test]
fn test() {
    // Arrange: maak twee bots aan
    let bot1 = Box::new(RandomBot::new());
    let bot2 = Box::new(RandomBot::new());
    let settings = MapConfig {
            bomb_timer: 4,
            bomb_radius: 3,
            endgame: 500,
            size: 7
        };

    // Start een spel met een kleine map zodat het snel klaar is
    let mut game = Game::build(  vec![bot1, bot2], settings, None);

    let x = game.run().replay_data;

    println!("Winnaar: {:?}", x);
    println!("Winnaar: {:?}", x);

}
