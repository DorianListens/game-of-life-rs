extern crate termion;
extern crate life;
use life::renderer::StringRenderer;
use life::board::grid::*;
use life::generator::*;
use std::io::stdout;

use std::time;

fn main() {
    let size = termion::terminal_size().unwrap_or((100, 100));

    let renderer = StringRenderer::new(stdout(), size.0, size.1);
    let board = GridBoard::random(size.0.into(), size.1.into());
    let generator = SimpleGenerator {};

    let game = life::Game::new(board, &renderer, &generator);
    //game.play(1000);
    game.play_with_delay(1000, time::Duration::from_millis(30));
}
