use crate::{hir_to_lir::state::State, ir::lir};

pub fn lower<'s>(state: &mut State, byte: u8) -> lir::node::Node<'s> {
    state.advance(1);
    lir::node::Node::Byte(byte)
}
