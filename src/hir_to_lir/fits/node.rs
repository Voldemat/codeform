use crate::ir::hir::node::Node;

use super::state::{MaybeWillFitState, NodeFitState};

pub fn does_node_fit<'s>(
    fit_state: MaybeWillFitState,
    node: &Node<'s>,
) -> NodeFitState {
    match node {
        Node::Byte(_) => super::byte::does_byte_fit(fit_state),
        Node::Line(line_mode) => {
            super::line::does_line_mode_fit(fit_state, *line_mode)
        }
        Node::AsciiOnelineText(token) => {
            super::text::does_ascii_text_fit(fit_state, token)
        }
        Node::Text(text) => {
            super::text::does_text_width_fit(fit_state, text.width)
        }
        Node::Tag(tag) => super::tag::process_tag(fit_state, tag),
    }
}
