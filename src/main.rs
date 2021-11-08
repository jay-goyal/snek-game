use pancurses::{
    curs_set, endwin, init_pair, initscr, noecho, start_color, Input, ACS_CKBOARD, COLOR_BLACK,
    COLOR_CYAN, COLOR_GREEN, COLOR_PAIR, COLOR_RED, COLOR_WHITE,
};
use rand::Rng;
mod snek;
use snek::{Direction, Food, Snake};

fn print_border(window: &pancurses::Window, my: i32, mx: i32) {
    for x in 0..mx + 1 {
        window.mvaddch(2, x, ACS_CKBOARD());
        window.mvaddch(my - 1, x, ACS_CKBOARD());
    }
    for y in 2..my + 1 {
        window.mvaddch(y, 0, ACS_CKBOARD());
        window.mvaddch(y, mx - 1, ACS_CKBOARD());
    }
}

fn main() {
    // Initialising Window
    let window = initscr();
    window.keypad(true);
    window.refresh();
    window.nodelay(true);
    curs_set(0);
    window.timeout(150);

    // Getting Window parameters
    let (my, mx) = window.get_max_yx();

    // Setting Colors
    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    init_pair(2, COLOR_RED, COLOR_BLACK);
    init_pair(3, COLOR_WHITE, COLOR_BLACK);
    init_pair(4, COLOR_CYAN, COLOR_BLACK);

    // Making screen background same
    for y in 0..my {
        for x in 0..mx {
            window.attron(COLOR_PAIR(1));
            window.mvaddch(y, x, ' ');
        }
    }

    // Initialising snake
    let mut snake = Snake::new(
        ((my / 2) as i32, (mx / 4) as i32),
        vec![
            ((my / 2) as i32, (mx / 4 - 1) as i32),
            ((my / 2) as i32, (mx / 4 - 2) as i32),
        ],
        Direction::RIGHT,
    );

    // Adding Food
    let mut food = Food::new((my / 2) as i32, (mx / 2) as i32);
    window.attron(COLOR_PAIR(2));
    food.draw_food(&window);

    // Score
    let mut score: u32 = 0;

    // Drawing border
    window.attron(COLOR_PAIR(3));
    print_border(&window, my, mx);

    // GAME LOOP
    loop {
        // Getting Keypress
        match window.getch() {
            Some(Input::KeyRight) => snake.change_direc(Direction::RIGHT),
            Some(Input::KeyLeft) => snake.change_direc(Direction::LEFT),
            Some(Input::KeyUp) => snake.change_direc(Direction::UP),
            Some(Input::KeyDown) => snake.change_direc(Direction::DOWN),
            Some(Input::Character('q')) => break,
            _ => (),
        }
        noecho();
        // Moving snake
        snake.move_snake(&window);

        // Printin score
        window.attron(COLOR_PAIR(3));
        window.mvprintw(1, 0, format!("SCORE: {}", score));
        window.mvprintw(1, mx - 15, format!("Press Q to quit"));

        // Checking if snake has eaten food
        if snake.check_food_eat(&food, &window) {
            score += 1;

            // Generating new food
            loop {
                food = Food::new(
                    rand::thread_rng().gen_range(3..my - 1),
                    rand::thread_rng().gen_range(1..mx - 1),
                );

                // Making sure food doesn't spawn in snake
                if !(snake.body.contains(&food.coord)) && !(food.coord == snake.head) {
                    break;
                }
            }

            snake.grow_snake();
            window.attron(COLOR_PAIR(2));
            food.draw_food(&window);
        }

        if (snake.head.0) == 2
            || (snake.head.0) == my - 1
            || (snake.head.1) == 0
            || (snake.head.1) == mx - 1
            || snake.body.contains(&snake.head)
        {
            break;
        }
    }
    endwin();
    println!("GAME OVER!!");
    println!("Your score was {}", score);
}
