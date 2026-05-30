/// Single line text element
pub struct Label {
    pub text: String,
}

impl Label {
    pub fn new(text: &str) -> Box<Self> {
        Box::new(Self {
            text: text.to_string(),
        })
    }
}
