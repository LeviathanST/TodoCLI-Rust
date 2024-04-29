use pancurses::*;

type Id = usize;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

#[derive(Default)]
pub struct UI {
    list_cur: Option<Id>,
    row: usize,
    col: usize,
}

impl UI {
    pub fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn begin_list(&mut self, id: Id) {
        self.list_cur = Some(id);
    }

    pub fn list_element(&mut self, window: &Window, label: &str, id: Id) {
        let id_cur = self
            .list_cur
            .expect("Not allowed to create list elements out side of lists");

        self.label(
            label,
            {
                if id_cur == id {
                    HIGHLIGHT_PAIR.into()
                } else {
                    REGULAR_PAIR.into()
                }
            },
            window,
        );
    }

    pub fn label(&mut self, text: &str, pair: i16, window: &Window) {
        window.mv(self.row.try_into().unwrap(), self.col.try_into().unwrap());
        window.attron(COLOR_PAIR(pair.try_into().unwrap()));
        window.addstr(text);
        window.attroff(COLOR_PAIR(pair.try_into().unwrap()));
        self.row += 1;
    }

    pub fn end_list(&mut self) {
        self.list_cur = None;
    }

    pub fn end(&mut self) {
        self.list_cur = None;
    }
}


