// entities.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Food,
    SnakeHead,
    SnakeBody,
}

pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![CellState::Empty; width as usize]; height as usize],
        }
    }

    pub fn update(&mut self, snake: &Snake, food: &Food) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell = CellState::Empty;
            }
        }

        self.cells[food.position().y as usize][food.position().x as usize] = CellState::Food;

        for &body_part in snake.body().iter() {
            self.cells[body_part.y as usize][body_part.x as usize] = CellState::SnakeBody;
        }

        let head = snake.head();
        self.cells[head.y as usize][head.x as usize] = CellState::SnakeHead;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> &Vec<Vec<CellState>> {
        &self.cells
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new(start: Point, direction: Direction) -> Self {
        Self {
            body: vec![start],
            direction,
        }
    }

    pub fn move_snake(&mut self) {
        let new_head = match self.direction {
            Direction::Up => Point::new(self.body[0].x, self.body[0].y.wrapping_sub(1)),
            Direction::Down => Point::new(self.body[0].x, self.body[0].y.wrapping_add(1)),
            Direction::Left => Point::new(self.body[0].x.wrapping_sub(1), self.body[0].y),
            Direction::Right => Point::new(self.body[0].x.wrapping_add(1), self.body[0].y),
        };

        self.body.insert(0, new_head);
        self.body.pop();
    }

    pub fn grow(&mut self) {
        let last = *self.body.last().unwrap();
        self.body.push(last);
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !self.is_opposite(new_direction) {
            self.direction = new_direction;
        }
    }

    fn is_opposite(&self, direction: Direction) -> bool {
        matches!(
            (self.direction, direction),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    pub fn head(&self) -> Point {
        self.body[0]
    }

    pub fn body(&self) -> &[Point] {
        &self.body
    }

    pub fn occupies(&self, point: &Point) -> bool {
        self.body.contains(point)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Food {
    position: Point,
}

impl Food {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn position(&self) -> Point {
        self.position
    }
}
