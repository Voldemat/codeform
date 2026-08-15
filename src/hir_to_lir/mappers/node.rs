use crate::{
    hir_to_lir::{config::Config, state::State},
    ir::{hir, lir},
};

pub fn lower<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [hir::node::Node<'s>],
    node: hir::node::Node<'s>,
) -> Option<lir::node::Node<'s>> {
    match node {
        hir::node::Node::Byte(byte) => Some(super::byte::lower(state, byte)),
        hir::node::Node::Line(line_mode) => {
            super::line::lower(config, state, line_mode)
        }
        hir::node::Node::AsciiOnelineText(ascii_text) => {
            Some(super::text::lower_ascii_text(state, ascii_text))
        }
        hir::node::Node::Text(unicode_text) => {
            Some(super::text::lower_unicode_text(config, state, unicode_text))
        }
        hir::node::Node::Tag(tag) => {
            super::tag::lower(config, state, get_next_nodes, tag)
        }
    }
}
