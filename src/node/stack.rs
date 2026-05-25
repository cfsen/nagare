use std::collections::HashMap;

use crate::node::node::{Node, NodeID, NodeIOKey};

#[derive(Clone)]
pub struct NodeStack {
    pub nodes: Vec<NodeMapping>,
    index: u16,
    id_idx_map: HashMap<NodeID, u16>,
}
impl Default for NodeStack {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            index: 0,
            id_idx_map: HashMap::new(),
        }
    }
}
impl NodeStack {
    pub fn add_node(&mut self, node: Node) -> NodeID {
        // TODO: umax checking, return result
        let id = NodeID(self.index);
        self.nodes.push(
            NodeMapping {
                id,
                node,
                input_map: Vec::<NodeRouting>::new(),
                output_map: Vec::<NodeRouting>::new(),
            }
        );
        self.id_idx_map.insert(id, self.index);
        self.index += 1;
        id
    }
    pub fn get_mapping(&self, node_id: &NodeID) -> Option<&NodeMapping> {
        let Some(&nodes_index) = self.id_idx_map.get(node_id) else { return None; };
        let Some(map) = self.nodes.get(nodes_index as usize) else { return None; };
        Some(map)
    }
    pub fn remove_node(&mut self, node_id: NodeID) {
        // TODO: return result
        // TODO: clean up links
        if let Some(i) = self.nodes.iter().position(|n| n.id == node_id) {
            self.nodes.remove(i);
        }
    }
    pub fn get_node(&self, node_id: NodeID) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == node_id).map(|m| &m.node)
    }
    pub fn get_mut_node(&mut self, node_id: NodeID) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == node_id).map(|m| &mut m.node)
    }
    pub fn get_node_mapping(&self, node_id: NodeID) -> Option<&NodeMapping> {
        self.nodes.iter().find(|n| n.id == node_id)
    }
    pub fn get_mut_node_mapping(&mut self, node_id: NodeID) -> Option<&mut NodeMapping> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }
    // TODO: hash lookup i: NodeID -> nodes[i] 

    // TODO: impl for input/output map mutation
    pub fn link_input(&mut self, route: NodeRouting) {}
    pub fn link_output(&mut self, route: NodeRouting) {}
    pub fn unlink_input(&mut self, route:  NodeRouting) {}
    pub fn unlink_output(&mut self, route: NodeRouting) {}
}

// This defines the connections of a node
#[derive(Clone)]
pub struct NodeMapping {
    pub id: NodeID,                     // must be unique for every node in the stack
    pub node: Node,                     // defines the node type
    pub input_map: Vec<NodeRouting>,    // defines how inputs are routed
    pub output_map: Vec<NodeRouting>,   // defines how outputs are routed
}
// Maps inputs/outputs to/from a node
#[derive(Clone)]
pub struct NodeRouting {
    node_id: NodeID,
    output: Option<NodeIOKey>,
    input: Option<NodeIOKey>,
}

