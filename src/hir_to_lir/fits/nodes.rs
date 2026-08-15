use super::state::{MaybeWillFitState, NodeFitState};

pub fn do_nodes_fit<'s>(
    fit_state: MaybeWillFitState,
    nodes: &[crate::ir::hir::node::Node<'s>],
) -> NodeFitState {
    nodes.iter().fold(
        NodeFitState::MaybeWillFit(fit_state),
        |current_fit_state, child_node| match current_fit_state {
            NodeFitState::DoesNotFit => current_fit_state,
            NodeFitState::DefinitelyFits => current_fit_state,
            NodeFitState::MaybeWillFit(maybe_will_fit_state) => {
                super::node::does_node_fit(maybe_will_fit_state, child_node)
            }
        },
    )
}
