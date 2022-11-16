use std::{
    default::{self, Default},
    env, fs, path,
};
const TAB_STOP: usize = 8;

pub struct Row {
    pub row_content: Box<String>,
    pub render: String,
}
impl Default for Row {
    fn default() -> Self {
        Self {
            row_content: Box::new(String::new()),
            render: String::new(),
        }
    }
}
impl Row {
    fn new(row_content: Box<String>, render: String) -> Self {
        Self {
            row_content,
            render,
        }
    }
    pub fn length(&self) -> usize {
        self.row_content.len().clone()
    }
}

pub struct EditorRows {
    row_contents: Vec<Row>,
    pub filename: Option<path::PathBuf>,
}
impl EditorRows {
    pub fn get_render(&self, at: usize) -> &String {
        &self.row_contents[at].render
    }

    pub fn get_editor_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }
    pub fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_contents: Vec::new(),
                filename: None,
            },
            Some(file) => Self::from_file(file.into()),
        }
    }
    fn from_file(file: path::PathBuf) -> Self {
        let file_contents = fs::read_to_string(&file).expect("Unable to read file");

        Self {
            filename: Some(file),
            row_contents: file_contents
                .lines()
                .map(|it| {
                    let res = format!("~ {}", it.replace("\t", "...."));
                    let mut row = Row::new(res.into(), String::new());
                    Self::render_row(&mut row);
                    row
                })
                .collect(),
        }
    }

    pub fn render_row(row: &mut Row) {
        // verefica la cantida de tab
        let mut index = 0;
        let capacity = row
            .row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_STOP } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                row.render.push(' ');
                while index % TAB_STOP != 0 {
                    row.render.push(' ');
                    index += 1
                }
            } else {
                row.render.push(c);
            }
        });
    }

    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub fn get_row(&self, at: usize) -> Result<&Row, ()> {
        let n = {
            let mut r = self.number_of_rows();
            if r > 0 {
                r -= 1
            }
            r
        };
        // &self.row_contents[at];
        if at < n || at == n {
            return Ok(&self.row_contents[at]);
        }
        Err(())
    }
}
