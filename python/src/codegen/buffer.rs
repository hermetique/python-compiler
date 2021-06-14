#![allow(unused)]
/// An ultra-simple buffer that maintains indentation in code. That's it.
pub struct Buffer {
    buffer: Vec<String>,
    indent: usize,
    level: usize,
}

impl Default for Buffer {
    /// Create a new buffer with an indentation level of 4.
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            indent: 4,
            level: 0,
        }
    }
}

impl ToString for Buffer {
    /// Dump the buffer to a string.
    fn to_string(&self) -> String {
        self.buffer.join("\n")
    }
}

impl Buffer {
    /// Write `i` to the buffer.
    pub fn write(&mut self, i: &str) {
        self.buffer
            .push(format!("{}{}", " ".repeat(self.indent * self.level), i));
    }

    /// Indent the buffer
    pub fn indent(&mut self) {
        self.level += 1;
    }

    /// Unindent the buffer.
    pub fn dedent(&mut self) {
        self.level -= 1;
    }

    pub fn dedent_level(&mut self, level: usize) {
        self.level -= level;
    }
}
