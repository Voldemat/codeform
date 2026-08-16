use crate::{
    hir_to_lir::{config::Config, fits, state::State},
    ir::{hir, lir, shared},
};

pub fn process_group<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [hir::node::Node<'s>],
    group: hir::tag::shared::Group,
) {
    let print_mode = match group.default_print_mode {
        crate::ir::shared::PrintMode::Expanded => {
            crate::ir::shared::PrintMode::Expanded
        }
        crate::ir::shared::PrintMode::Flat => fits::nodes::do_nodes_fit(
            fits::state::MaybeWillFitState {
                remaining_width: (config.max_width.get())
                    .saturating_sub(state.current_line_width),
                group_depth: 0,
            },
            get_next_nodes(),
        )
        .to_print_mode(),
    };
    state.states_stack.push(
        state
            .states_stack
            .last()
            .unwrap()
            .clone()
            .with_expected_end_tag_kind(Some(hir::tag::EndTagKind::Group))
            .with_print_mode(print_mode),
    );

    if let Some(group_id) = group.id {
        state.group_mode_map.insert(group_id, print_mode);
    };
}

pub fn process_conditional_group<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [hir::node::Node<'s>],
    conditional_group: hir::tag::shared::ConditionalGroup,
) {
    // 1. Look up the mode of the target group in state
    let condition_met = state
        .group_mode_map
        .get(&conditional_group.condition.target_group_id)
        .map_or(false, |&target_mode| {
            target_mode == conditional_group.condition.expected_mode
        });
    let mut new_state = state
        .states_stack
        .last()
        .unwrap()
        .clone()
        .with_expected_end_tag_kind(Some(
            hir::tag::EndTagKind::ConditionalGroup,
        ));

    if condition_met {
        let print_mode = match conditional_group.default_print_mode {
            shared::PrintMode::Expanded => shared::PrintMode::Expanded,
            shared::PrintMode::Flat => fits::nodes::do_nodes_fit(
                fits::state::MaybeWillFitState {
                    remaining_width: (config.max_width.get())
                        .saturating_sub(state.current_line_width),
                    group_depth: 0,
                },
                get_next_nodes(),
            )
            .to_print_mode(),
        };
        new_state = new_state.with_print_mode(print_mode);
    }
    state.states_stack.push(new_state);
}

pub fn lower_start_tag_kind<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [hir::node::Node<'s>],
    start_tag: hir::tag::StartTagKind,
) -> Option<lir::tag::StartTagKind> {
    match start_tag {
        hir::tag::StartTagKind::Group(group) => {
            process_group(config, state, get_next_nodes, group);
            None
        }
        hir::tag::StartTagKind::ConditionalGroup(conditional_group) => {
            process_conditional_group(
                config,
                state,
                get_next_nodes,
                conditional_group,
            );
            None
        }
        hir::tag::StartTagKind::Indent(indent_mode) => {
            let mut new_state = 
                    state
                        .states_stack
                        .last()
                        .unwrap()
                        .clone()
                        .with_expected_end_tag_kind(Some(
                            hir::tag::EndTagKind::Indent,
                        ));
            if indent_mode == hir::tag::IndentMode::Hard
                || state.active_mode() == shared::PrintMode::Expanded
            {
                new_state.indent_level += 1;
                state.states_stack.push(new_state);
                Some(lir::tag::StartTagKind::Indent)
            } else {
                new_state.enabled = false;
                state.states_stack.push(new_state);
                None
            }
        }
    }
}

pub fn lower_end_tag_kind(
    state: &mut State,
    end_tag_kind: hir::tag::EndTagKind,
) -> Option<lir::tag::EndTagKind> {
    let last_state = state.states_stack.pop().unwrap();
    assert_eq!(last_state.expected_end_tag_kind, Some(end_tag_kind));

    match end_tag_kind {
        hir::tag::EndTagKind::Group => None,
        hir::tag::EndTagKind::Indent => {
            if last_state.enabled {
                Some(lir::tag::EndTagKind::Indent)
            } else {
                None
            }
        }
        hir::tag::EndTagKind::ConditionalGroup => None,
    }
}

pub fn lower<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [hir::node::Node<'s>],
    tag: hir::tag::Tag,
) -> Option<lir::node::Node<'s>> {
    match tag {
        hir::tag::Tag::Start(start_tag_kind) => {
            lower_start_tag_kind(config, state, get_next_nodes, start_tag_kind)
                .map(|start_tag_kind| {
                    lir::node::Node::Tag(lir::tag::Tag::Start(start_tag_kind))
                })
        }
        hir::tag::Tag::End(end_tag_kind) => {
            lower_end_tag_kind(state, end_tag_kind).map(|end_tag_kind| {
                lir::node::Node::Tag(lir::tag::Tag::End(end_tag_kind))
            })
        }
    }
}
