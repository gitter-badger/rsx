#![feature(plugin)]
#![plugin(rsx)]

mod example_dom;
use example_dom::{Node, NodeKind};
use example_dom::button::ButtonBuilder;
use example_dom::window::WindowBuilder;

fn main() {
    let app_root = rsx!(<Window width=1200 height=600>
                            <Button content="Hello">
                            </Button>
                            <Button content="Hi">
                            </Button>
                        </Window>);
                        
    println!("{:#?}", app_root);
}