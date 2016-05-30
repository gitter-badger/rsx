extern crate aster;

use syntax::ast::Expr;
use syntax::ptr::P;
use syntax::parse::token::Lit;
use self::aster::AstBuilder;
use self::aster::expr::ExprBuilder;

use super::Node;
use super::Content;

fn build_node(node: &Node) -> Result<P<Expr>, String> {
    let builder = AstBuilder::new();
    let builder_path_segment = format!("{}Builder", &node.name.as_str());
    
    let node_path = builder.expr().path()
                       .segment(builder_path_segment).build()
                       .segment("new").build()
                   .build();
    let node_expr = builder.expr().call().build(node_path);
    
    let mut e = node_expr.build();
    for arg in &node.attributes {
        let arg_expr = match arg.1 { 
            Lit::Integer(num) => {
                let num_str : &str = &num.as_str();
                
                builder.expr().u32(num_str.parse::<u32>().unwrap())
            },
            Lit::Str_(s) => {
                builder.expr().call().build(
                    builder.expr().path()
                        .segment("String").build()
                        .segment("from").build()
                    .build()    
                ).with_arg(builder.expr().str(s)).build()
            },
            _ => panic!("")
        };
        
        e = ExprBuilder::new()
                .method_call(arg.0)
                .build(e)
                .arg().build(arg_expr)
                .build();
    }
    
    let builder_output = ExprBuilder::new().method_call("build").build(e).build();
    let node_box_expr = builder.expr().box_().build(builder_output);
                            
    let node_kind = builder.expr().call().build(
                        builder.expr().path()
                            .segment("NodeKind").build()
                            .segment(node.name).build()
                        .build()).with_arg(node_box_expr).build();
    Ok(node_kind)
}

pub fn build(node: &Node) -> Result<P<Expr>, String> {
    let node_type_expr = try!(build_node(node));
    
    let builder = AstBuilder::new();
    let node_path = builder.expr().path()
                        .segment("Node").build()
                        .segment("new").build()
                        .build();
    let node_id = builder.expr().usize(1);
  
    let mut child_exprs = vec![];
    for c in &node.children {
        if let &Content::Node(ref n) = c {
            let child_expr = try!(build(n));
            child_exprs.push(child_expr);   
        } 
    }
    
    let empty_vec = ExprBuilder::new().vec().with_exprs(child_exprs).build();
    
    let node = builder.expr().call().build(
        node_path
    ).with_arg(node_id).with_arg(node_type_expr).with_arg(empty_vec).build();
    
    Ok(node)
}