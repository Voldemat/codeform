use crate::{
    decider::{config::Config, state::State},
    dom::node::DOMNode,
};

pub fn decide_nodes<'s>(
    config: &Config,
    state: &mut State,
    nodes: &[DOMNode<'s>],
) {
    for (index, node) in nodes.iter().enumerate() {
        let next_nodes = nodes.get(index + 1..).unwrap_or_default();
        super::node::decide_node(config, state, || next_nodes, node);
    }
}
