#![crate_name="rsx"] 
#![crate_type="dylib"]
#![feature(plugin, plugin_registrar, rustc_private, quote)]
extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

mod parser;

use parser::Parser;
use syntax::codemap::Span;
use syntax::ast::TokenTree;
use syntax::ext::base::{MacResult, DummyResult, MacEager, ExtCtxt};
use rustc_plugin::Registry;

fn compile_rsx(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let mut p = Parser::new(args.iter());
    
    match p.parse_node() {
        Err(msg) => {
            cx.span_err(sp, &msg); 
            return DummyResult::expr(sp); 
        },
        Ok(root_element) => as_expr(root_element).unwrap()
    }
}

pub fn as_expr(node: parser::Node) -> Result<Box<MacResult + 'static>, String> {
    Ok(MacEager::expr(try!(parser::build(&node))))
}

#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_macro("rsx", compile_rsx);
}