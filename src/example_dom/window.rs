#[derive(Debug)]
pub struct Window {
    width: u32,
    height: u32
}

pub struct WindowBuilder {
    width: u32,
    height: u32
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            width: 800,
            height: 600
        }
    }
    
    pub fn build(self) -> Window {
        Window {
            width: self.width,
            height: self.height
        }
    }
    
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
}