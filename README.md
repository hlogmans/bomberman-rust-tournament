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



== Run with web gui:
With docker:
    - docker compose up watch
    or
    - docker compose up release

Without docker:
    Install cargo-leptos and add web assembly support:
    - install perl
        (for fedora linux) sudo dnf install -y perl pkg-config openssl-devel gcc make
    - cargo install --locked cargo-leptos
    - rustup target add wasm32-unknown-unknown

    Run with hot reload (watch does not work on windows):
    - cd ./web
    - cargo leptos watch

    Run with better performance:
    - cargo leptos build --release
    - cargo leptos serve --release


    
cargo install trunk
rustup target add wasm32-unknown-unknown

start backend
cargo run -p backend --release

start web
trunk serve --open

start cli 
cargo run --release