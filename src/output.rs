use crate::cursor::CursorController;
use crate::editormod::{EditorContents, EditorRows};
use crossterm::event::*;
use crossterm::terminal::ClearType;
use crossterm::{execute, queue, style, terminal};
use std::cmp;
use std::io::{self, Write};

use text2art::{BasicFonts, Font, Printer};

const VERSION: &str = "1.0.0";
fn welcome() -> Vec<String> {
    let mut res = Vec::new();
    let t1 = "Welcome to text-editor".to_string();
    res.push(t1);
    res.push(VERSION.clone().to_string());
    res
}
struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

pub struct Output {
    pub win_size: (usize, usize),
    pub editor_contents: EditorContents,
    pub cursor_controller: CursorController,
    pub editor_rows: EditorRows,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 1)) // modify
            .unwrap();

        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
            editor_rows: EditorRows::new(),
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(io::stdout(), terminal::Clear(ClearType::All))?;
        execute!(io::stdout(), crossterm::cursor::MoveTo(0, 0))
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        let binding = welcome();
        let mut welcome = binding.into_iter();
        for i in 0..screen_rows {
            let file_row = i + self.cursor_controller.row_offset;

            if file_row >= self.editor_rows.number_of_rows() {
                /* add this line */
                if self.editor_rows.number_of_rows() == 0 && i == screen_rows / 3 {
                    for i in 0..welcome.len() {
                        let mut padding = (screen_columns - welcome.len()) / 2;

                        if padding != 0 {
                            self.editor_contents.push('~');
                            padding -= 1
                        }

                        (0..padding).for_each(|_| self.editor_contents.push(' '));
                        self.editor_contents.push_str(&welcome.next().expect(""));
                    }
                    /* format!("Pound Editor --- Version {} ", VERSION);

                         if welcome.len() > screen_columns {
                             welcome.truncate(screen_columns)
                          }

                         let mut padding = (screen_columns - welcome.len()) / 2;

                          if padding != 0 {
                              self.editor_contents.push('~');
                              padding -= 1
                          }

                          (0..padding).for_each(|_| self.editor_contents.push(' '));
                         self.editor_contents.push_str(&welcome);
                    */
                } else {
                    self.editor_contents.push('~');
                }
                /* add the following*/
            } else {
                let row = self.editor_rows.get_render(file_row);
                let column_offset = self.cursor_controller.column_offset;
                let len = cmp::min(row.len().saturating_sub(column_offset), screen_columns);
                let start = if len == 0 { 0 } else { column_offset };
                // self.editor_contents.push_str("~ ");
                //&format!("~ {}", &row[start..start + len])
                self.editor_contents.push_str(&row[start..start + len]) //modify
            }

            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            //    if i < screen_rows - 1 {
            self.editor_contents.push_str("\r\n");
            //  }
        }
    }
    fn draw_status_bar(&mut self) {
        self.editor_contents
            .push_str(&style::Attribute::Reverse.to_string());
        (0..self.win_size.0).for_each(|_| self.editor_contents.push(' '));
        self.editor_contents
            .push_str(&style::Attribute::Reset.to_string());
        let info = format!("1");
    }
    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller
            .move_cursor(direction, &self.editor_rows);
    }
    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        self.cursor_controller.scroll(&self.editor_rows);
        Self::clear_screen()?;
        queue!(
            self.editor_contents,
            crossterm::cursor::Hide,
            terminal::Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.cursor_x - self.cursor_controller.column_offset;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;
        queue!(
            self.editor_contents,
            crossterm::cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            crossterm::cursor::Show
        )?;
        // self.draw_status_bar();
        self.editor_contents.flush()?;
        Ok(())
    }
}
