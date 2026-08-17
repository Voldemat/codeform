use super::state::{MaybeWillFitState, NodeFitState};

pub fn process_start_tag(
    fit_state: MaybeWillFitState,
    start_tag: &crate::ir::hir::tag::StartTagKind,
) -> NodeFitState {
    match start_tag {
        crate::ir::hir::tag::StartTagKind::Group(_)
        | crate::ir::hir::tag::StartTagKind::ConditionalGroup(_) => {
            NodeFitState::MaybeWillFit(fit_state.with_incremented_group_depth())
        }
        crate::ir::hir::tag::StartTagKind::Indent(
            crate::ir::hir::tag::IndentMode::Hard,
        ) => NodeFitState::DefinitelyFits,
        _ => NodeFitState::MaybeWillFit(fit_state),
    }
}

pub fn process_end_tag_kind(
    fit_state: MaybeWillFitState,
    end_tag_kind: &crate::ir::hir::tag::EndTagKind,
) -> NodeFitState {
    match end_tag_kind {
        crate::ir::hir::tag::EndTagKind::Group
        | crate::ir::hir::tag::EndTagKind::ConditionalGroup => {
            if fit_state.group_depth == 0 {
                NodeFitState::DefinitelyFits
            } else {
                NodeFitState::MaybeWillFit(
                    fit_state.with_decremented_group_depth(),
                )
            }
        }
        _ => NodeFitState::MaybeWillFit(fit_state),
    }
}

pub fn process_tag(
    fit_state: MaybeWillFitState,
    tag: &crate::ir::hir::tag::Tag,
) -> NodeFitState {
    match tag {
        crate::ir::hir::tag::Tag::Start(start_tag) => {
            process_start_tag(fit_state, start_tag)
        }
        crate::ir::hir::tag::Tag::End(end_tag_kind) => {
            process_end_tag_kind(fit_state, end_tag_kind)
        }
        crate::ir::hir::tag::Tag::ExpandParent => NodeFitState::DoesNotFit,
    }
}
