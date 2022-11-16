mod cursor;
mod editormod;
mod output;
mod reader;
use crossterm::*;
use editormod::Editor;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    /* modify */
    let mut editor = Editor::new();
    while editor.run()? {}
    /* end */
    Ok(())
}
