use enum_dispatch::enum_dispatch;
use crate::{id_type, node::{error::NodeError, node_thresholdboolean::ThresholdBoolean}};

id_type!(NodeID);
id_type!(NodeIOKey);

// master list of nodes available
#[derive(Clone)]
#[enum_dispatch(NodeBehavior,NodeInput,NodeOutput)]
pub enum Node {
    Bool(ThresholdBoolean),
}

// Defines the node
#[derive(Clone)]
pub struct NodeBase {
    pub display_name: String,       // for user customization
    pub node_type_name: String,     // default name of node type
    pub inputs: Vec<NodeIO>,        // ordered list of input params and type
    pub outputs: Vec<NodeIO>,       // ordered list of outputs and type
    pub params: Vec<NodeIO>,        // ordered list of user configurable params
}
impl NodeBase {
    fn get_io(&self, chan: NodeIOChannel, key: NodeIOKey) -> Option<&NodeIO> {
        match chan {
            NodeIOChannel::Input => { self.inputs.iter().find(|n| n.key == key) },
            NodeIOChannel::Output => { self.outputs.iter().find(|n| n.key == key) },
            NodeIOChannel::Param => { self.params.iter().find(|n| n.key == key) },
        }
    }
    fn get_mut_io(&mut self, chan: NodeIOChannel, key: NodeIOKey) -> Option<&mut NodeIO> {
        match chan {
            NodeIOChannel::Input => { self.inputs.iter_mut().find(|n| n.key == key) },
            NodeIOChannel::Output => { self.outputs.iter_mut().find(|n| n.key == key) },
            NodeIOChannel::Param => { self.params.iter_mut().find(|n| n.key == key) },
        }
    }
    fn set_io(&mut self, chan: NodeIOChannel, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError> {
        let Some(n) = self.get_mut_io(chan, key) else { return Err(NodeError::SetIONodeNotFound); };
        if value.as_ref().is_some_and(|v| v.kind() != n.kind) { 
            // TODO: log and warn
            return Err(NodeError::SetIOKindMismatch);
        };
        n.value = value;
        Ok(())
    }

    // Macro from here on
    pub fn get_input(&self, key: NodeIOKey) -> Option<&NodeIO> {
        self.get_io(NodeIOChannel::Input, key)
    }
    pub fn get_input_value(&self, key: NodeIOKey) -> Option<&NodeValue> {
        self.get_io(NodeIOChannel::Input, key)
            .and_then(|i| i.value.as_ref())
    }
    pub fn set_input(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError> {
        self.set_io(NodeIOChannel::Input, key, value)
    }
    pub fn reset_inputs(&mut self) {
        for ele in self.inputs.iter_mut() {
            ele.value = None;
        }
    }

    pub fn get_output(&self, key: NodeIOKey) -> Option<&NodeIO> {
        self.get_io(NodeIOChannel::Output, key)
    }
    pub fn get_output_value(&self, key: NodeIOKey) -> Option<&NodeValue> {
        self.get_io(NodeIOChannel::Output, key)
            .and_then(|o| o.value.as_ref())
    }
    pub fn set_output(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError>{
        self.set_io(NodeIOChannel::Output, key, value)
    }
    pub fn reset_outputs(&mut self) {
        for ele in self.outputs.iter_mut() {
            ele.value = None;
        }
    }

    pub fn get_param(&self, key: NodeIOKey) -> Option<&NodeIO> {
        self.get_io(NodeIOChannel::Param, key)
    }
    pub fn get_param_value(&self, key: NodeIOKey) -> Option<&NodeValue> {
        self.get_io(NodeIOChannel::Param, key)
            .and_then(|p| p.value.as_ref())
    }
    pub fn set_param(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError> {
        self.set_io(NodeIOChannel::Param, key, value)
    }
    pub fn reset_params(&mut self) {
        for ele in self.params.iter_mut() {
            ele.value = None;
        }
    }
    pub fn reset(&mut self) {
        self.reset_inputs();
        self.reset_outputs();
        self.reset_params();
    }
}
pub enum NodeIOChannel {
    Input,
    Output,
    Param,
}

#[derive(Clone)]
pub struct NodeIO {
    pub key: NodeIOKey,
    pub kind: NodeIOKind,
    pub value: Option<NodeValue>,
}
impl NodeIO {
    pub fn new_empty(key: NodeIOKey, kind: NodeIOKind) -> Self {
        Self { key, kind, value: None }
    }
}

// schema for node values
#[derive(Clone, PartialEq)]
pub enum NodeIOKind {
    String,
    Integer,
    Float,
    Bool,
}
#[derive(Clone, PartialEq, PartialOrd)]
pub enum NodeValue {
    String(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
}
impl NodeValue {
    pub fn kind(&self) -> NodeIOKind {
        match self {
            NodeValue::String(_) => NodeIOKind::String,
            NodeValue::Integer(_) => NodeIOKind::Integer,
            NodeValue::Float(_) => NodeIOKind::Float,
            NodeValue::Bool(_) => NodeIOKind::Bool,
        }
    }
}

// common node behaviors
pub trait NodeBehavior {
    fn run(&mut self) -> Result<NodeRunResult, NodeError>;
    fn get_node_base(&self) -> &NodeBase;
    fn reset(&mut self);
}

pub trait NodeDefault {
    fn default_input_map() -> Vec<NodeIO>;
    fn default_output_map() -> Vec<NodeIO>;
    fn default_param_map() -> Vec<NodeIO>;
}

pub trait NodeInput {
    fn set_input(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError>;
    fn get_input(&self, key: NodeIOKey) -> Option<&NodeValue>;
    fn get_input_map(&self) -> &Vec<NodeIO>;
    fn reset_inputs(&mut self);
}
pub trait NodeOutput {
    fn set_output(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError>;
    fn get_output(&self, key: NodeIOKey) -> Option<&NodeValue>;
    fn get_output_map(&self) -> &Vec<NodeIO>;
    fn reset_outputs(&mut self);
}
pub trait NodeParam {
    fn set_param(&mut self, key: NodeIOKey, value: Option<NodeValue>) -> Result<(), NodeError>;
    fn get_param(&self, key: NodeIOKey) -> Option<&NodeValue>;
    fn reset_params(&mut self);
}

// abstraction for results
pub enum NodeRunResult {
    Success,
    Failed(String), // TODO: return error enum
}
