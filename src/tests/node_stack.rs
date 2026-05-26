#[cfg(test)]
mod tests {
    use crate::node::{
        node::Node, 
        node_thresholdboolean::ThresholdBoolean,
        stack::NodeStack
    };

    fn make_node() -> Node {
        Node::Bool(ThresholdBoolean::default())
    }

    #[test]
    fn stack_create() {
        let stack = NodeStack::default();
        assert_eq!(stack.nodes.len(), 0);
    }

    #[test]
    fn stack_add_node() {
        let mut stack = NodeStack::default();
        let id = stack.add_node(make_node());
        assert_eq!(stack.nodes.len(), 1);
        assert_eq!(stack.nodes[0].id, id);
    }

    #[test]
    fn stack_remove_node() {
        let mut stack = NodeStack::default();
        let id = stack.add_node(make_node());
        assert_eq!(stack.nodes.len(), 1);
        stack.remove_node(id);
        assert_eq!(stack.nodes.len(), 0);
    }

    #[test]
    fn stack_get_mut_node() {
        let mut stack = NodeStack::default();
        let id = stack.add_node(make_node());

        let node = stack.get_mut_node(id);
        assert!(node.is_some());

        let fake_id = crate::node::node::NodeID(999);
        let missing = stack.get_mut_node(fake_id);
        assert!(missing.is_none());
    }

    #[test]
    fn stack_get_node_mapping() {
        let mut stack = NodeStack::default();
        let id = stack.add_node(make_node());

        let mapping = stack.get_node_mapping(id);
        assert!(mapping.is_some());
        assert_eq!(mapping.unwrap().id, id);

        let fake_id = crate::node::node::NodeID(999);
        assert!(stack.get_node_mapping(fake_id).is_none());
    }

    #[test]
    fn stack_get_mut_node_mapping() {
        let mut stack = NodeStack::default();
        let id = stack.add_node(make_node());

        let mapping = stack.get_mut_node_mapping(id);
        assert!(mapping.is_some());
        assert_eq!(mapping.unwrap().id, id);

        let fake_id = crate::node::node::NodeID(999);
        assert!(stack.get_mut_node_mapping(fake_id).is_none());
    }
}
