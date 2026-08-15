use super::state::{MaybeWillFitState, NodeFitState};

pub fn does_line_mode_fit(
    fit_state: MaybeWillFitState,
    line_mode: crate::ir::hir::node::LineMode,
) -> NodeFitState {
    match line_mode {
        crate::ir::hir::node::LineMode::Hard
        | crate::ir::hir::node::LineMode::Empty => NodeFitState::DefinitelyFits,
        crate::ir::hir::node::LineMode::SoftOrSpace => {
            if fit_state.remaining_width < 1 {
                NodeFitState::DoesNotFit
            } else {
                NodeFitState::MaybeWillFit(
                    fit_state.with_decreased_remaining_width(1),
                )
            }
        }
        crate::ir::hir::node::LineMode::Soft => {
            NodeFitState::MaybeWillFit(fit_state)
        }
    }
}
