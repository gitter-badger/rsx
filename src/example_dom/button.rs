#[derive(Debug)]
pub struct Button {
    content: String
}

pub struct ButtonBuilder {
    content: Option<String> 
}

impl ButtonBuilder {
    pub fn new() -> Self {
        ButtonBuilder {
            content: None
        }
    }
    
    pub fn build(self) -> Button {
        Button {
            content: self.content.unwrap()
        }
    }
    
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
}