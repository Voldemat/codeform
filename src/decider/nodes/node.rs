use crate::{
    decider::{config::Config, state::State},
    dom::node::DOMNode,
};

use super::{
    line_mode::decide_line_mode, tag::decide_tag,
    text_width::decide_text_width, token::decide_token,
};

pub fn decide_node<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [DOMNode<'s>] + Clone + Copy,
    node: &DOMNode<'s>,
) {
    match node {
        DOMNode::Space => state.advance(1),
        DOMNode::Line(line_mode) => {
            decide_line_mode(config, state, state.active_mode(), *line_mode)
        }
        DOMNode::Text(text) => decide_text_width(config, state, text.width),
        DOMNode::Token(token) => decide_token(state, token),
        DOMNode::Tag(tag) => decide_tag(config, state, get_next_nodes, tag),
    }
}
