Bomberman Using Rust Project - BURP

== Goals

This project is intended as a small project to discover the main features and language idiom of Rust. The main idea is 
that participants write bots, improve bots and let the system run tournaments to determine the best bot.

== What is Rust

You know C++? C#? Great, Rust provides a good way to get you finally into productive mode, especially if you are working with threads or low level code.

== How to create a bot
To create a new bot, add a new file under `/bots/src/bot`, implement the `Bot` trait for your bot inside that file.
You can use the template file `template_bot.rs` as a starting point.

== How to install
Install rust from the rust website
Then run those commands in the terminal:
 - rustup install nightly
 - rustup override set nightly

 == How to build
- cargo build

== How to run
- cargo run --release

== Run with web gui:
Install Trunk and add the wasm target
- cargo install trunk
- rustup target add wasm32-unknown-unknown

Start the frontend
- cd web
- trunk serve --open

Start the backend in a new terminal (Only required to run tournaments)
- cargo run -p backend --release