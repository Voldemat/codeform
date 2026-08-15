use crate::{
    hir_to_lir::{config::Config, state::State},
    ir::{hir, lir, shared},
};

pub fn lower<'s>(
    config: &Config,
    state: &mut State,
    line_mode: hir::node::LineMode,
) -> Option<lir::node::Node<'s>> {
    match (line_mode, state.active_mode()) {
        (hir::node::LineMode::Hard | hir::node::LineMode::Empty, _)
        | (
            hir::node::LineMode::SoftOrSpace | hir::node::LineMode::Soft,
            shared::PrintMode::Expanded,
        ) => {
            state.reset_line(config.indent_width);
            None
        }
        (hir::node::LineMode::SoftOrSpace, shared::PrintMode::Flat) => {
            state.advance(1);
            Some(lir::node::Node::Byte(b' '))
        }
        (hir::node::LineMode::Soft, shared::PrintMode::Flat) => None,
    }
}
