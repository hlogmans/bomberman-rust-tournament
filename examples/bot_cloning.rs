use rust_bomberman::bot::{available_bots, clone_bot};

fn main() {
    println!("Bot Cloning Example");
    println!("===================");

    // Create some bots using the available constructors
    let bot_constructors = available_bots();

    // Create original bots
    let original_random_bot = bot_constructors[0]("RandomBot Original");
    let original_easy_bot = bot_constructors[1]("EasyBot Original");

    println!("Original bots created:");
    println!("- {}", original_random_bot.name());
    println!("- {}", original_easy_bot.name());

    // Clone the bots using the clone_bot function
    let cloned_random_bot = clone_bot(original_random_bot.as_ref());
    let cloned_easy_bot = clone_bot(original_easy_bot.as_ref());

    println!("\nCloned bots:");
    println!("- {}", cloned_random_bot.name());
    println!("- {}", cloned_easy_bot.name());

    // Demonstrate that they are separate instances
    println!("\nDemonstrating separate instances:");
    println!(
        "Original RandomBot address: {:p}",
        original_random_bot.as_ref()
    );
    println!("Cloned RandomBot address: {:p}", cloned_random_bot.as_ref());

    // Show that cloning preserves the bot's properties
    println!("\nBoth bots have the same name format:");
    println!("Original: {}", original_random_bot.name());
    println!("Cloned: {}", cloned_random_bot.name());

    // Example of using cloned bots in a game setup
    println!("\nExample: Creating multiple games with cloned bots");

    // Create bots for multiple games
    let game1_bots = vec![
        clone_bot(original_random_bot.as_ref()),
        clone_bot(original_easy_bot.as_ref()),
    ];

    let game2_bots = vec![
        clone_bot(original_random_bot.as_ref()),
        clone_bot(original_easy_bot.as_ref()),
    ];

    println!("Game 1 bots:");
    for bot in &game1_bots {
        println!("- {}", bot.name());
    }

    println!("Game 2 bots:");
    for bot in &game2_bots {
        println!("- {}", bot.name());
    }

    println!("\nBot cloning allows you to:");
    println!("1. Create multiple instances of the same bot type");
    println!("2. Run parallel games with separate bot states");
    println!("3. Implement tournament systems where bots can compete multiple times");
    println!("4. Save and restore bot configurations");
}
