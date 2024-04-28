use std::cmp::min;

use pancurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    let window = initscr();

    start_color();
    noecho();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    curs_set(0);

    window.refresh();
    window.keypad(true);

    let todo_list: Vec<String> = vec![
        "Hello world with Rust".to_string(),
        "Love KH :3".to_string(),
        "Love KH :3".to_string(),
        "Love KH :3".to_string(),
    ];

    let mut done = Vec::<String>::new();

    let mut todo_cur: usize = 0;

    let mut quit: bool = false;
    while !quit {
        for (i, todo) in todo_list.iter().enumerate() {
            let pair: i16 = {
                if todo_cur == i {
                    HIGHLIGHT_PAIR.into()
                } else {
                    REGULAR_PAIR.into()
                }
            };

            window.attron(COLOR_PAIR(pair.try_into().unwrap()));
            window.mv(i as i32, 1);
            window.addstr(&todo);
            window.attroff(COLOR_PAIR(pair.try_into().unwrap()));
        }

        match window.getch() {
            Some(Input::Character('q')) => quit = true,
            Some(Input::Character('w')) => {
                if (todo_cur > 0) {
                    todo_cur -= 1
                }
            }
            Some(Input::Character('s')) => {
                if (todo_cur < todo_list.len() - 1) {
                    todo_cur = min(todo_cur + 1, todo_list.len());
                }
            }
            _ => {}
        }
    }

    endwin();
}
