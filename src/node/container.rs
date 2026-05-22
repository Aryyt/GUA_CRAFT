use std::error::Error;

use crate::{node::Node, renderer::Renderer};

/// A container is a node designed to hold multiple other nodes.
///
/// # Remarks
///
/// Events are forwarded to each child node in order, following the expected semantics.
///
/// All children will have their update/draw functions called in order, during the update/draw process of the container.
pub struct Container {
    nodes: Vec<Box<dyn Node>>,
}

impl Container {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node<T>(&mut self, node: T)
    where
        T: Node,
    {
        self.nodes.push(Box::new(node));
    }

    pub fn remove_node(&mut self, index: usize) -> Box<dyn Node> {
        self.nodes.remove(index)
    }

    pub fn get_node(&self, index: usize) -> Option<&dyn Node> {
        match self.nodes.len() >= index {
            true => None,
            false => Some(self.nodes[index].as_ref()),
        }
    }

    pub fn remove_where<F>(&mut self, f: F) -> Option<Box<dyn Node>>
    where
        F: Fn(&dyn Node) -> bool,
    {
        for i in 0..self.nodes.len() {
            let node = self.nodes[i].as_ref();

            if f(node) {
                return Some(self.nodes.remove(i));
            }
        }

        None
    }

    pub fn get_where<F>(&mut self, f: F) -> Option<&dyn Node>
    where
        F: Fn(&dyn Node) -> bool,
    {
        for node in &self.nodes {
            let node = node.as_ref();

            if f(node) {
                return Some(node);
            }
        }

        None
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Container {
    fn handle_input(&self, input: &sdl3::event::Event) -> bool {
        for node in &self.nodes {
            if node.as_ref().handle_input(input) {
                break;
            }
        }

        false
    }

    fn update(&self, dt: std::time::Duration) {
        for node in &self.nodes {
            node.as_ref().update(dt);
        }
    }

    fn draw(&self, renderer: &dyn Renderer) -> Result<(), Box<dyn Error>> {
        for node in &self.nodes {
            node.as_ref().draw(renderer)?;
        }

        Ok(())
    }
}
