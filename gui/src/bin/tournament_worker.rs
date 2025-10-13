use gui::tournament_worker::TournamentWorker;

use gloo::worker::Registrable;

fn main() {
    console_error_panic_hook::set_once();

    TournamentWorker::registrar().register();
}