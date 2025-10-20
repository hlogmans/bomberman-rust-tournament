Bomberman Using Rust Project - BURP

= Goals

This project is intended as a small project to discover the main features and language idiom of Rust. The main idea is 
that participants write bots, improve bots and let the system run tournaments to determine the best bot.

= What is Rust

You know C++? C#? Great, Rust provides a good way to get you finally into productive mode, especially if you are working with threads or low level code.

== How to install
Install rust from the rust website
Then run those commands in the terminal:
 - rustup install nightly
 - rustup override set nightly

 == How to build
- cargo build

== How to run
- cargo run --release --bin rust-bomberman



== Web
Install cargo-leptos and add web assembly support:
- cargo install --locked cargo-leptos
- rustup target add wasm32-unknown-unknown

Run with hot reload:
- cd ./web
- cargo leptos watch

Run with better performance:
- cargo leptos build --release
- cargo leptos serve --release
