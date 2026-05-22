use guacraft::{
    gamehost::GameHost,
    node::{Node, clearnode::ClearNode, container::Container},
};
use sdl3::pixels::Color;

fn main() {
    println!("阿弥陀佛!");

    let mut host = GameHost::new().unwrap();

    let mut container = Container::new();

    container.add_node(
        ClearNode::new()
            .with_size((800, 300))
            .with_position((0, 300))
            .with_clear_colour(Color::RGB(0, 255, 155)),
    );

    container.add_node(
        ClearNode::new()
            .with_size((800, 300))
            .with_clear_colour(Color::RGB(255, 0, 100)),
    );

    host.set_root_node(container);

    host.run().unwrap();
}
