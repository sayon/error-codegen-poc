pub struct PrettyPrinter {
    indent: usize,
    buffer: String,
}
impl PrettyPrinter {
    pub fn new(capacity: usize) -> Self {
        let buffer = String::with_capacity(capacity);
        let indent = 0;
        Self { indent, buffer }
    }
    pub fn get_buffer(self) -> String {
        self.buffer
    }
    pub fn indent_increase(&mut self) {
        self.indent_increase_by(1);
    }
    pub fn indent_increase_by(&mut self, offset: usize) {
        self.indent += offset;
    }
    pub fn indent_decrease_by(&mut self, offset: usize) {
        self.indent -= offset;
    }
    pub fn indent_decrease(&mut self) {
        self.indent_decrease_by(1);
    }

    pub fn indent(&mut self) {
        for _ in 0..self.indent {
            self.buffer.push_str("   ");
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.buffer.push_str(string);
    }

    pub fn push_line(&mut self, string: &str) {
        self.indent();
        self.buffer.push_str(string);
        self.buffer.push_str("\n");
    }

    pub fn push_block(&mut self, string: &str) {
        for line in string.lines() {
            self.push_line(line);
        }
    }
}

const DEFAULT_CAPACITY: usize = 1024 * 1024;

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}
