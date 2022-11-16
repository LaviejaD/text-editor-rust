use std::cmp::{self, Ordering};

use crate::editormod::{EditorRows, Row};
use crossterm::event::KeyCode;

const TAB_STOP: usize = 8;
pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    screen_columns: usize,
    screen_rows: usize,
    pub row_offset: usize,
    pub column_offset: usize,
    pub render_x: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            cursor_x: 2,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
            row_offset: 0,
            column_offset: 0,
            render_x: 0,
        }
    }
    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        let number_of_rows = {
            let mut res = editor_rows.number_of_rows();
            if res > 0 {
                res -= 1
            }
            res
        };
        match direction {
            KeyCode::Up => {
                if self.cursor_y != 0 {
                    self.cursor_y = self.cursor_y.saturating_sub(1);
                }
            }

            KeyCode::Down => {
                if self.cursor_y != self.screen_rows - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = (editor_rows.get_row(self.cursor_y).expect("a")).length();
                }
            }
            KeyCode::Right => {
                let legnth = (editor_rows.get_row(self.cursor_y).expect("a")).length();
                if self.cursor_y < number_of_rows {
                    match self.cursor_x.cmp(&legnth) {
                        Ordering::Less => self.cursor_x += 1,
                        Ordering::Equal => {
                            self.cursor_y += 1;
                            self.cursor_x = 2
                        }
                        _ => unimplemented!(),
                    }
                }
            }
            KeyCode::PageUp => {
                if self.cursor_y != 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::End => {
                if self.cursor_y < number_of_rows {
                    self.cursor_x = (editor_rows.get_row(self.cursor_y).expect("a")).length();
                }
            }
            KeyCode::Home => self.cursor_x = 2,
            _ => todo!(),
        }
    }

    pub fn scroll(&mut self, editor_rows: &EditorRows) {
        self.render_x = 0;
        if self.cursor_y < editor_rows.number_of_rows() {
            self.render_x = self.get_render_x(editor_rows.get_editor_row(self.cursor_y));
        }
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
        self.column_offset = cmp::min(self.column_offset, self.render_x); //modify
        if self.render_x >= self.column_offset + self.screen_columns {
            //modify
            self.column_offset = self.render_x - self.screen_columns + 1; //modify
        }
    }
    fn get_render_x(&self, row: &Row) -> usize {
        row.row_content[..self.cursor_x]
            .chars()
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + (TAB_STOP - 1) - (render_x % TAB_STOP) + 1
                } else {
                    render_x + 1
                }
            })
    }
}
