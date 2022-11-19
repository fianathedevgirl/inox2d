use glam::Vec2;
use serde::{Serialize, Deserialize, Serializer};

use crate::nodes::node::{NodeState, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplePhysics {
    node_state: NodeState,
    param: u32,
    model_type: String,
    map_mode: String,
    gravity: f32,
    length: f32,
    frequency: f32,
    angle_damping: f32,
    length_damping: f32,
    output_scale: Vec2,
}

impl<S: Serializer> Node<S> for SimplePhysics {
    fn get_node_state(&self) -> &NodeState {
        &self.node_state
    }

    fn get_node_state_mut(&mut self) -> &mut NodeState {
        &mut self.node_state
    }

    fn serialize_node(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize(serializer)
    }
}