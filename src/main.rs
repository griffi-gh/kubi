mod game;
mod logging;

fn main() {
    logging::init();
    game::run();
}
