use super::state::{MaybeWillFitState, NodeFitState};

pub fn does_width_fit(
    fit_state: MaybeWillFitState,
    width: crate::ir::hir::Width,
) -> NodeFitState {
    if width.value() > fit_state.remaining_width {
        NodeFitState::DoesNotFit
    } else {
        NodeFitState::MaybeWillFit(
            fit_state.with_decreased_remaining_width(width.value()),
        )
    }
}

pub fn does_text_width_fit(
    fit_state: MaybeWillFitState,
    text_width: crate::ir::hir::TextWidth,
) -> NodeFitState {
    match text_width {
        crate::ir::hir::TextWidth::Width(w) => does_width_fit(fit_state, w),
        crate::ir::hir::TextWidth::Multiline => NodeFitState::DoesNotFit,
    }
}

pub fn does_ascii_text_fit<'s>(
    fit_state: MaybeWillFitState,
    token: &'s str,
) -> NodeFitState {
    if token.len() as u32 > fit_state.remaining_width {
        NodeFitState::DoesNotFit
    } else {
        NodeFitState::MaybeWillFit(
            fit_state.with_decreased_remaining_width(token.len() as u32),
        )
    }
}
