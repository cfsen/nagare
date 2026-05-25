use crate::{node::{error::NodeError, node::{NodeBase, NodeBehavior, NodeDefault, NodeIO, NodeIOKey, NodeIOKind, NodeInput, NodeOutput, NodeParam, NodeRunResult, NodeValue}}, node_impl};

//
// All nodes must impl the traits:
// Default
// NodeDefault
// NodeBehavior
// NodeInput*
// NodeOutput*
// NodeParam*
//
// * Can be implemented with the node_impl macro when
// no additional customization is required.
//

#[derive(Clone)]
pub struct ThresholdBoolean {
    base: NodeBase,  // required for NodeBehavior trait
}
impl Default for ThresholdBoolean {
    fn default() -> Self {
        let inputs = Self::default_input_map();
        let outputs = Self::default_output_map();
        let params = Self::default_param_map();

        Self {
            base: NodeBase { 
                display_name: format!("Boolean"),
                node_type_name: format!("Boolean"),
                inputs,
                outputs,
                params,
            }
        }
    }
}
// node_impl!(ThresholdBoolean);   // NOTE: see #node_impl_macro_note

impl NodeDefault for ThresholdBoolean {
    fn default_input_map() -> Vec<NodeIO> {
        let mut inputs = Vec::<NodeIO>::new(); 
        inputs.push(
            NodeIO::new_empty(NodeIOKey(0), NodeIOKind::Integer)
        );
        inputs
    }
    fn default_output_map() -> Vec<NodeIO> {
        let mut outputs = Vec::<NodeIO>::new();
        outputs.push(
            NodeIO::new_empty(NodeIOKey(0), NodeIOKind::Bool)
        );
        outputs
    }
    fn default_param_map() -> Vec<NodeIO> {
        let mut params = Vec::<NodeIO>::new();
        params.push(
            NodeIO::new_empty(NodeIOKey(0), NodeIOKind::Float)
        );
        params
    }
}
impl NodeBehavior for ThresholdBoolean {
    fn get_node_base(&self) -> &NodeBase {
        &self.base
    }
    fn run(&mut self) -> Result<NodeRunResult, NodeError> {
        self.reset_outputs();

        let input = self.get_input(NodeIOKey(0))
            .ok_or_else(|| NodeError::GetInput("Failed to get input 0".to_string()))?;

        let threshold = self.get_param(NodeIOKey(0))
            .ok_or_else(|| NodeError::GetParam("Failed to get param 0".to_string()))?;

        self.set_output(NodeIOKey(0), Some(NodeValue::Bool(input <= threshold)))?;

        Ok(NodeRunResult::Success)
    }
    fn reset(&mut self) {
        self.base.reset();
    }
}

// TODO: update macro
// NOTE: #node_impl_macro_note
// The node_impl macro implements boilerplate heavy traits that shouldn't usually need
// customization.
impl NodeInput for ThresholdBoolean {
    fn set_input(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError> {
        self.base.set_input(key, value)
    }
    fn get_input(&self, key: NodeIOKey) -> Option<&NodeValue> {
        self.base.get_input_value(key)
    }
    fn get_input_map(&self) -> &Vec<NodeIO> {
        &self.base.inputs
    }
    fn reset_inputs(&mut self) {
        self.base.reset_inputs();
    }
}
impl NodeOutput for ThresholdBoolean {
    fn set_output(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError> {
        self.base.set_output(key, value)
    }
    fn get_output(&self, key: NodeIOKey) -> Option<&NodeValue> {
        self.base.get_output_value(key)
    }
    fn get_output_map(&self) -> &Vec<NodeIO> {
        &self.base.outputs
    }
    fn reset_outputs(&mut self) {
        self.base.reset_outputs();
    }
}
impl NodeParam for ThresholdBoolean {
    fn set_param(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError> {
        self.base.set_param(key, value)
    }
    fn get_param(&self, key: NodeIOKey) -> Option<&NodeValue> {
        self.base.get_param(key).map_or(None, |p| p.value.as_ref())
    }
    fn reset_params(&mut self) {
        self.base.reset_params();
    }
}
