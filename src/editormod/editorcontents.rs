const TAB_STOP: usize = 8;

use std::io;

pub struct EditorContents {
    content: String,
}

impl EditorContents {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    pub fn push_str(&mut self, txt: &str) {
        self.content.push_str(txt)
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(io::stdout(), "{}", self.content);
        io::stdout().flush()?;
        self.content.clear();
        out
    }
}
