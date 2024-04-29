mod ui;

use std::cmp::min;

use pancurses::*;

use ui::UI;

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

    let mut todo_list: Vec<String> = vec![
        "Hello world with Rust".to_string(),
        "Love KH :3".to_string(),
        "Love KH :3".to_string(),
        "Love KH :3".to_string(),
    ];

    let mut done_list: Vec<String> =
        vec!["Hello world with Rust".to_string(), "Make todo".to_string()];

    let mut done_cur: usize = 0;
    let mut todo_cur: usize = 0;

    let mut ui = UI::default();
    let mut quit: bool = false;
    while !quit {
        window.erase();
        ui.begin(0, 0);
        {
            ui.label("TODO: ", REGULAR_PAIR, &window);
            ui.begin_list(todo_cur);
            for (i, todo) in todo_list.iter().enumerate() {
                ui.list_element(&window, &format!("- [ ] {}", todo), i);
            }
            ui.end_list();

            ui.label(
                "--------------------------------------------------",
                REGULAR_PAIR,
                &window,
            );

            ui.label("DONE: ", REGULAR_PAIR, &window);
            ui.begin_list(done_cur);
            for (i, done) in done_list.iter().enumerate() {
                ui.list_element(&window, &format!("- [x] {}", done), i);
            }

            ui.end_list();
        }
        ui.end();

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
            Some(Input::Character(' ')) => {
                if (todo_cur >= 0) {
                    todo_list.remove(todo_cur);
                }
                done_list.push(todo_list[todo_cur].clone());
            }
            _ => {}
        }
    }

    endwin();
}
