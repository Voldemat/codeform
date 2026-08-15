use super::state::{MaybeWillFitState, NodeFitState};

pub fn does_byte_fit(fit_state: MaybeWillFitState) -> NodeFitState {
    if fit_state.remaining_width < 1 {
        NodeFitState::DoesNotFit
    } else {
        NodeFitState::MaybeWillFit(fit_state.with_decreased_remaining_width(1))
    }
}
