use std::{any::Any, error::Error, time::Duration};

use crate::renderer::Renderer;

/// A node contains functionality to provide update and draw behaviour, typically called recursively by a higher level gamehost.
pub trait Node: Any {
    /// Propogates events to the node, which may choose to handle it
    ///
    /// # Returns
    /// Whether the input is handled, and therefore blocked from further propogation.
    fn handle_input(&self, _input: &sdl3::event::Event) -> bool {
        false
    }

    /// Updates the internal state of the node.
    ///
    /// # Remarks
    /// `update` should be used if the internal state of the node is modified by the node itself.
    fn update(&self, _dt: Duration) {}

    /// Draws the node using the provided renderer.
    ///
    /// # Guidelines
    /// Nodes should implement their draw behaviour with the "leave no trace" philosophy, ensuring that the renderer state is restored before the function returns.
    ///
    /// Sibling nodes should not interfere with each other, while child nodes can be affected by the parent node's rendering scopes.
    ///
    /// # Remarks
    /// There are no limits to what behaviour is implemented by a node.
    fn draw(&self, _renderer: &dyn Renderer) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub mod clearnode;
pub mod container;
