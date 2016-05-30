mod node;
mod build;

use syntax::ast::{Ident, Name, TokenTree};
use syntax::parse::token::{Token, Lit, BinOpToken};
use std::iter::Peekable;

pub use self::build::build as build;
pub use self::node::Node as Node;
pub use self::node::Content as Content;

pub struct Parser<'t, I: Iterator<Item=&'t TokenTree>> {
    current: Option<&'t TokenTree>,
    look_ahead: Option<&'t TokenTree>,
    token_iterator: Peekable<I>
}

impl<'t, I: Iterator<Item=&'t TokenTree>> Parser<'t, I> {
    pub fn new(iterator: I) -> Self {
        let mut iterator = iterator.peekable();
        
        Parser {
            current: iterator.next(),
            look_ahead: iterator.peek().map(|t| *t),
            token_iterator: iterator
        }    
    }
    
    fn advance(&mut self) {
        let tok = self.token_iterator.next();
        self.current = tok;
        self.look_ahead = self.token_iterator.peek().map(|t| *t);
    }
    
    fn expect(&mut self, token: Token) -> Result<Token, String> {
        if let Some(&TokenTree::Token(_, ref tok)) = self.current {
            if *tok == token {
                self.advance();
                Ok(tok.clone())
            } else {
                Err(format!("expected {:?} found {:?}", token, tok))    
            }
        } else {
            Err(format!("expected {:?}, found something else", token))
        } 
    }

    fn expect_nc(&mut self, token: Token) -> Result<Token, String> {
        if let Some(&TokenTree::Token(_, ref tok)) = self.current {
            if *tok == token {
                Ok(tok.clone())
            } else {
                Err(format!("expected {:?} found {:?}", token, tok))    
            }
        } else {
            Err(format!("expected {:?}, found something else", token))
        } 
    }
        
    fn expect_ident(&mut self) -> Result<Ident, String> {
        if let Some(&TokenTree::Token(_, ref tok)) = self.current {
            if let &Token::Ident(ident) = tok {
                self.advance();
                Ok(ident.clone())
            } else {
                Err(format!("expected ident found {:?}", tok))    
            }
        } else {
            Err(format!("expected ident, found something else"))
        } 
    }
    
    fn expect_binop(&mut self, binop: BinOpToken) -> Result<Token, String> {
        if let Some(&TokenTree::Token(_, ref tok)) = self.current {
            match tok {
                &Token::BinOp(bo) => {
                    if bo == binop {
                        self.advance();
                        Ok(tok.clone())
                    } else {
                        Err(format!("expected binop, found {:#?}", bo))
                    }
                },
                _ => Err(format!("expected binop, found {:#?}", tok))
            }
        } else {
            Err(format!("expected binop, found something else"))
        } 
    }
    
    // Look ahead variant, ugly and should be changed.
    fn expect_binop_la(&mut self, binop: BinOpToken) -> Result<Token, String> {
        if let Some(&TokenTree::Token(_, ref tok)) = self.look_ahead {
            match tok {
                &Token::BinOp(bo) => {
                    if bo == binop {
                        Ok(tok.clone())
                    } else {
                        Err(format!("expected binop, found {:#?}", bo))
                    }
                },
                _ => Err(format!("expected binop, found {:#?}", tok))
            }
        } else {
            Err(format!("expected binop, found something else"))
        } 
    }
    
    fn parse_open_tag(&mut self) -> Result<(Name, Vec<(Ident, Lit)>), String> {
        try!(self.expect(Token::Lt));
        let name = try!(self.expect_ident()).name;
        let mut attrs = vec![];
        
        while let Some(_) = self.current {
            if self.expect(Token::Gt).is_ok() {                
                break;
            } else {
                let attr_name = try!(self.expect_ident());
                try!(self.expect(Token::Eq));
                
                let attr_val = { 
                    if let &TokenTree::Token(_, ref val) = self.current.unwrap() {                    
                        if let &Token::Literal(lit, _) = val {
                            self.advance();
                            lit       
                        } else {
                            unimplemented!()
                        }
                    } else {
                        unimplemented!()
                    }
                };
                
                attrs.push((attr_name, attr_val));
            }
        }
        
        Ok((name, attrs))
    }
    
    fn parse_body(&mut self) -> Result<Vec<node::Content>, String> {
        let mut children = vec![];
        
        while let Some(_) = self.current {
            if self.expect_nc(Token::Lt).is_ok() {
                if self.expect_binop_la(BinOpToken::Slash).is_ok() {
                    break;
                } else {
                    let node = try!(self.parse_node());
                    children.push(node::Content::Node(node));
                }
            } else {
                unimplemented!()
            }
        }
        
        Ok(children)
    }
    
    fn parse_close_tag(&mut self) -> Result<(), String> {
        try!(self.expect(Token::Lt));
        try!(self.expect_binop(BinOpToken::Slash));
        
        try!(self.expect_ident());
        
        // TODO: check that names match against the opening tag.
        try!(self.expect(Token::Gt));
        
        Ok(())
    }
    
    pub fn parse_node(&mut self) -> Result<Node, String> {
        let (node_name, attributes) = try!(self.parse_open_tag());
        
        let children = try!(self.parse_body());
        try!(self.parse_close_tag());
        
        Ok(Node::new(node_name, attributes, children))
    }
}