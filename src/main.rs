mod constant;
mod ui;
mod utils;

use pancurses::*;

use constant::focus::FOCUS;
use ui::UI;
use utils::list::{list_down, list_remove, list_up};

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

    let mut ui: UI = UI::default();
    let mut quit: bool = false;
    let mut focus: FOCUS = FOCUS::DONE;

    while !quit {
        window.erase();
        ui.begin(0, 0);
        {
            match focus {
                FOCUS::TODO => {
                    ui.label("TODO: ", REGULAR_PAIR, &window);
                    ui.begin_list(todo_cur);
                    for (i, todo) in todo_list.iter().enumerate() {
                        ui.list_element(&window, &format!("- [ ] {}", todo), i);
                    }
                    ui.end_list();
                }
                FOCUS::DONE => {
                    ui.label("DONE: ", REGULAR_PAIR, &window);
                    ui.begin_list(done_cur);
                    for (i, done) in done_list.iter().enumerate() {
                        ui.list_element(&window, &format!("- [x] {}", done), i);
                    }

                    ui.end_list();
                }
            }
        }
        ui.end();

        match window.getch() {
            Some(Input::Character('q')) => quit = true,
            Some(Input::Character('w')) => match focus {
                FOCUS::TODO => list_up(&mut todo_cur),
                FOCUS::DONE => list_up(&mut done_cur),
            },
            Some(Input::Character('s')) => match focus {
                FOCUS::TODO => list_down(&mut todo_list, &mut todo_cur),
                FOCUS::DONE => list_down(&mut done_list, &mut done_cur),
            },
            Some(Input::Character('t')) => focus = focus.toggle(),
            Some(Input::Character('\n')) => match focus {
                FOCUS::TODO => list_remove(&mut todo_list, &mut todo_cur, &mut done_list),
                FOCUS::DONE => list_remove(&mut done_list, &mut done_cur, &mut todo_list),
            },
            _ => {}
        }
    }

    endwin();
}
