mod app;
mod home;
mod game;
mod components;

use app::App;
use leptos::{logging, mount};

pub fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");
    mount::mount_to_body(App);
}