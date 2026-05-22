use guacraft::{
    gamehost::GameHost,
    node::{Node, clearnode::ClearNode, container::Container},
};
use sdl3::pixels::Color;

fn main() {
    println!("阿弥陀佛!");

    let mut host = GameHost::new().unwrap();

    host.set_root_node(
        ClearNode::new()
            .with_size((800, 300))
            .with_clear_colour(Color::RGB(255, 0, 100)),
    );
    host.run().unwrap();
}
