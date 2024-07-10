// game.rs
use crate::entities::{Board, Direction, Food, Point, Snake};
use crate::input::InputHandler;
use crate::rendering::render;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Game {
    board: Board,
    snake: Snake,
    food: Food,
    input_handler: InputHandler,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::new(10, 10);
        let snake = Snake::new(Point::new(5, 5), Direction::Left);
        let mut rng = rand::thread_rng();
        let food = Food::new(Point::new(rng.gen_range(0..10), rng.gen_range(0..10)));

        Self {
            board,
            snake,
            food,
            input_handler: InputHandler::new(),
            rng,
        }
    }

    pub fn update(&mut self) -> bool {
        if let Some(direction) = self.input_handler.get_direction() {
            self.snake.change_direction(direction);
        }

        if self.input_handler.should_quit() {
            return false;
        }

        self.snake.move_snake();

        if self.snake.head() == self.food.position() {
            self.snake.grow();
            self.spawn_new_food();
        }

        self.board.update(&self.snake, &self.food);

        true
    }

    pub fn render(&self) {
        render(&self.board);
        println!("Snake Info: {:?}", self.snake);
        println!("Food Info: {:?}", self.food);
    }

    fn spawn_new_food(&mut self) {
        loop {
            let new_position = Point::new(
                self.rng.gen_range(0..self.board.width()),
                self.rng.gen_range(0..self.board.height()),
            );
            if !self.snake.occupies(&new_position) {
                self.food = Food::new(new_position);
                break;
            }
        }
    }
}
