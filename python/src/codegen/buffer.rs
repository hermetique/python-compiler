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

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string());

        Ok(())
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Buffer {}

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

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn test_buffer_default() {
        let buffer = Buffer::default();
        assert_eq!(buffer.level, 0);
        assert_eq!(buffer.indent, 4);
    }

    #[test]
    fn test_buffer_write() {
        let mut buffer = Buffer::default();
        buffer.write("test");

        assert_eq!(buffer.to_string(), "test");
    }

    #[test]
    fn test_indent_works() {
        let mut buffer = Buffer::default();

        buffer.indent();
        buffer.write("test");

        assert_eq!(
            format!("{}test", " ".repeat(buffer.indent * buffer.level)),
            buffer.to_string()
        );
    }

    #[test]
    fn test_dedent_works() {
        let mut buffer = Buffer::default();

        let expected = "test\n\ttest";

        buffer.write("test");
        buffer.indent();
        buffer.write("test");
    }

    #[test]
    fn test_multiline_indent_works() {
        let expected = "    test\n    test";

        let mut buffer = Buffer::default();
        buffer.indent();
        buffer.write("test");
        buffer.write("test");

        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn test_buffer_eq() {
        let mut buffer = Buffer::default();
        buffer.write("Test");

        assert!(Buffer::default() != buffer);
        assert!("Test" == buffer.to_string());
    }
}
