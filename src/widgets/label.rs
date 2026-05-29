pub struct Label {
    pub text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}
