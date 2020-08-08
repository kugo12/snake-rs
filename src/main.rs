use raylib::prelude::*;

use std::ops;
use rand::Rng;
use rand::prelude::ThreadRng;


const WINDOW_HEIGHT: i32 = 400;
const WINDOW_WIDTH: i32 = 400;
const WINDOW_TITLE: &str = "Snake";
const TILE_SIZE: i32 = 20;
const TILES_W: i32 = WINDOW_WIDTH / TILE_SIZE - 1;
const TILES_H: i32 = WINDOW_HEIGHT / TILE_SIZE - 1;


#[derive(PartialEq, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn bound(&mut self){
        if self.x > TILES_W {
            self.x = 0;
        } else if self.x < 0 {
            self.x = TILES_W;
        }
        if self.y > TILES_H {
            self.y = 0;
        } else if self.y < 0 {
            self.y = TILES_H;
        }
    }

    fn random_in_bound(mut rng: ThreadRng) -> Position {
        Position {
            x: rng.gen_range(0, TILES_W),
            y: rng.gen_range(0, TILES_H)
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position; 

    fn add(self, val: Position) -> Position {
        let mut pos = Position {
            x: self.x + val.x,
            y: self.y + val.y
        };
        pos.bound();
        pos
    }
}

#[derive(Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right
}

impl Move {
    fn pos(&self) -> Position {
        match *self {
            Move::Up => Position {x: 0, y: -1},
            Move::Down => Position {x: 0, y: 1},
            Move::Left => Position {x: -1, y: 0},
            Move::Right => Position {x: 1, y: 0}
        }
    }

    fn key(&self) -> KeyboardKey {
        match *self {
            Move::Up => KeyboardKey::KEY_W,
            Move::Down => KeyboardKey::KEY_S,
            Move::Left => KeyboardKey::KEY_A,
            Move::Right => KeyboardKey::KEY_D
        }
    }

    fn iter() -> impl Iterator<Item = &'static Move> {
        use Move::*;

        [Up, Down, Left, Right].iter()
    }
}

struct Game {
    snake: Vec<Position>,
    dead: bool,
    direction: Move,
    apple: Position,
}

impl Game {
    fn init() -> Game {
        Game {
            snake: vec![
                Position {x: 10, y: 10},
                Position {x: 11, y: 10},
                Position {x: 12, y: 10}
            ],
            dead: false,
            direction: Move::Left,
            apple: Position {x: 4, y: 11}
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .build();

    let mut frame_count = 0;
    let rng = rand::thread_rng();

    let mut game = Game::init();
    let mut next_pos: Position;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if game.dead {
            if rl.is_key_down(KeyboardKey::KEY_SPACE) {
                game = Game::init();
            }
        }
        for dir in Move::iter() {
            if rl.is_key_down(dir.key()) {
                if game.snake[0] + dir.pos() != game.snake[1]{
                    game.direction = *dir;
                }
            }
        }

        if frame_count == 10 {
            next_pos = game.snake[0] + game.direction.pos();
            game.dead = game.snake.contains(&next_pos);
            game.snake.insert(0, next_pos);

            if game.snake[0] == game.apple {
                while game.snake.contains(&game.apple) {
                    game.apple = Position::random_in_bound(rng);
                }
            } else {
                game.snake.pop();
            }
            frame_count = 0;
        }

        let mut d: raylib::core::drawing::RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        if game.dead {
            d.draw_text(&format!("Your score is {}", game.snake.len() - 3), 20, 20, 20, Color::WHITE);
            d.draw_text("Press space to restart", 20, WINDOW_HEIGHT - 40, 20, Color::WHITE);
        } else {
            d.draw_rectangle(game.apple.x * TILE_SIZE, game.apple.y * TILE_SIZE, TILE_SIZE, TILE_SIZE, Color::RED);
            for square in game.snake.iter() {
                d.draw_rectangle(square.x * TILE_SIZE, square.y * TILE_SIZE, TILE_SIZE, TILE_SIZE, Color::GREEN);
            }

            frame_count += 1;
        }
    }
}
