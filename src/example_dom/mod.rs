pub mod button;
pub mod window;

#[derive(Debug)]
pub enum NodeKind {
    Button(Box<button::Button>),
    Window(Box<window::Window>)
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub kind: NodeKind,
    pub children: Vec<Node>
}

impl Node {
    pub fn new(id: usize, kind: NodeKind, children: Vec<Node>) -> Self {
        Node {
            id: id,
            kind: kind,
            children: children
        }
    }
}
