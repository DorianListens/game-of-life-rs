extern crate termion;
extern crate life;
use life::models::*;
use life::renderer::ScreenRenderer;
use life::board::square::*;
use life::interface::*;
use life::generator::*;
use std::io::stdout;

use std::{thread, time};

fn main() {
    let size = termion::terminal_size().expect("Couldn't get terminal size");

    let renderer = ScreenRenderer::new(stdout(), size.1, size.0);
    let board = GridBoard::random(size.1.into(), size.0.into());
    let generator = SquareGenerator {};

    let game = life::Game::new(board, &renderer, &generator);
    game.play(100);
    // game.play_with_delay(100, time::Duration::from_millis(50));
}
