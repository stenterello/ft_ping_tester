#[derive(Debug, Clone)]
pub enum TextType {
    Standard(Vec<String>),
    Formatted(Vec<Vec<(bool, u8)>>),
}

#[derive(Debug, Default)]
pub enum TestResult {
    Correct,
    Incorrect,
    #[default]
    Unknown,
}
