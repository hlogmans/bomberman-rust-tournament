use bots::random_bot::RandomBot;
use game::game::game::Game;

#[test]
fn integration_game_runs_and_has_winner() {
    // Arrange: maak twee bots aan
    let bot1 = Box::new(RandomBot::new());
    let bot2 = Box::new(RandomBot::new());

    // Start een spel met een kleine map zodat het snel klaar is
    let mut game = Game::build( Some(7),  Some(7), vec![bot1, bot2], None);

    // Act: speel maximaal 100 rondes of tot er een winnaar is
    let mut rounds = 0;
    while game.winner.is_none() && rounds < 1000 {
        game.run_round(None, None, None);
        rounds += 1;
    }

    // Assert: er moet een winnaar zijn
    assert!(
        game.winner.is_some(),
        "Het spel zou binnen 100 rondes een winnaar moeten hebben"
    );
    let winner_name = game.winner_name();
    assert!(
        winner_name.is_some(),
        "Er zou een winnaar-naam moeten zijn als het spel klaar is"
    );
    println!("Winnaar: {:?}", winner_name);
}

#[test]
fn test() {
    // Arrange: maak twee bots aan
    let bot1 = Box::new(RandomBot::new());
    let bot2 = Box::new(RandomBot::new());

    // Start een spel met een kleine map zodat het snel klaar is
    let mut game = Game::build(Some(7),  Some(7), vec![bot1, bot2], None);

    let x = game.run().replay_data;

    println!("Winnaar: {:?}", x);
    println!("Winnaar: {:?}", x);

}
