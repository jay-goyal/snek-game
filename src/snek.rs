use pancurses::{ACS_BLOCK, ACS_CKBOARD, COLOR_PAIR};

pub struct Food {
    pub coord: (i32, i32),
}

impl Food {
    pub fn new(y: i32, x: i32) -> Food {
        Food { coord: (y, x) }
    }

    pub fn draw_food(&self, window: &pancurses::Window) {
        window.mvaddch(self.coord.0, self.coord.1, ACS_BLOCK());
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opp(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    pub head: (i32, i32),
    pub body: Vec<(i32, i32)>,
    direction: Direction,
}

impl Snake {
    pub fn new(head: (i32, i32), body: Vec<(i32, i32)>, direction: Direction) -> Snake {
        Snake {
            head,
            body,
            direction,
        }
    }

    pub fn change_direc(&mut self, direction: Direction) {
        if (self.direction.opp() != direction) && (self.direction != direction) {
            self.direction = direction;
        }
    }

    pub fn move_snake(&mut self, window: &pancurses::Window) {
        let mut tmp: Vec<(i32, i32)> = vec![self.head];
        let tail = self.body.pop().unwrap();
        tmp.append(&mut self.body);
        self.body = tmp;
        match self.direction {
            Direction::Up => self.head = (self.head.0 - 1, self.head.1),
            Direction::Down => self.head = (self.head.0 + 1, self.head.1),
            Direction::Left => self.head = (self.head.0, self.head.1 - 1),
            Direction::Right => self.head = (self.head.0, self.head.1 + 1),
        }

        window.attron(COLOR_PAIR(4));
        window.mvaddch(self.head.0, self.head.1, ACS_CKBOARD());
        window.attron(COLOR_PAIR(1));
        for s in &self.body {
            window.mvaddch(s.0, s.1, ACS_CKBOARD());
        }
        window.mvaddch(tail.0, tail.1, ' ');
    }

    pub fn check_food_eat(&mut self, food: &Food) -> bool {
        if self.head == food.coord {
            return true;
        }
        false
    }

    pub fn grow_snake(&mut self) {
        self.body.push(*(self.body.last().unwrap()));
    }
}
