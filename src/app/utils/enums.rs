#[derive(Debug, Clone)]
pub enum TextType {
    Standard(Vec<String>),
    Formatted(Vec<Vec<(bool, u8)>>),
}

impl TextType {
    pub fn clear(&mut self) -> () {
        match self {
            TextType::Standard(vec) => vec.clear(),
            TextType::Formatted(vec) => vec.clear()
        }
    }
}

#[derive(Debug, Default)]
pub enum TestResult {
    Correct,
    Incorrect,
    #[default]
    Unknown,
}
