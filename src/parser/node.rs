use syntax::ast::{Name, Expr, Ident};
use syntax::parse::token::{Lit};

#[derive(Debug)]
pub struct Node {
    pub name: Name,
    pub attributes: Vec<(Ident, Lit)>,
    pub children: Vec<Content>
}

#[derive(Debug)]
pub enum Content {
    Node(Node),
    Literal(Lit),
    Expr(Expr)
}

impl Node {
    pub fn new(name: Name, attributes: Vec<(Ident, Lit)>, children: Vec<Content>) -> Self {
        Node {
            name: name,
            attributes: attributes,
            children: children
        }
    }
}