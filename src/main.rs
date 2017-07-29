extern crate life;
use life::models::*;
use life::renderer::ScreenRenderer;
use life::board::square::*;
use life::interface::*;
use life::generator::*;

use std::{thread, time};

fn main() {
    let renderer = ScreenRenderer::new(50);
    let board = SquareBoard::random(50);
    let generator = SquareGenerator {};

    let game = life::Game::new(board, &renderer, &generator);
    game.play_with_delay(100, time::Duration::from_millis(50));
}
