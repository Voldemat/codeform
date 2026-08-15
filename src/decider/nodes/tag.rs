use crate::{
    decider::{config::Config, fits, state::State},
    dom::{
        node::DOMNode,
        tag::{
            EndTagKind, StartTagKind, Tag,
            shared::{ConditionalGroup, Group, GroupMode},
        },
    },
    shared::PrintMode,
};

pub fn decide_group<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [DOMNode<'s>],
    group: &Group,
) {
    let print_mode = match group.mode.get() {
        GroupMode::Expand => PrintMode::Expanded,
        GroupMode::Flat => fits::do_nodes_fit(
            fits::MaybeWillFitState {
                remaining_width: (config.max_width.get())
                    .saturating_sub(state.current_line_width),
                group_depth: 0,
            },
            get_next_nodes(),
        )
        .to_print_mode(),
    };
    group.mode.set(print_mode.into());
    state.mode_stack.push(print_mode);

    if let Some(group_id) = group.id {
        state.group_mode_map.insert(group_id, print_mode);
    };
}

pub fn decide_conditional_group<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [DOMNode<'s>],
    conditional_group: &ConditionalGroup,
) {
    // 1. Look up the mode of the target group in state
    let condition_met = state
        .group_mode_map
        .get(&conditional_group.condition.target_group_id)
        .map_or(false, |&target_mode| {
            target_mode == conditional_group.condition.expected_mode
        });

    // 2. Evaluate print mode: check `fits` ONLY if the condition was met
    let print_mode = if condition_met {
        match conditional_group.mode.get() {
            GroupMode::Expand => PrintMode::Expanded,
            GroupMode::Flat => fits::do_nodes_fit(
                fits::MaybeWillFitState {
                    remaining_width: (config.max_width.get())
                        .saturating_sub(state.current_line_width),
                    group_depth: 0,
                },
                get_next_nodes(),
            )
            .to_print_mode(),
        }
    } else {
        // If the condition fails, force Expand without running expensive fitting logic
        PrintMode::Expanded
    };

    // 3. Update internal node mode and state mode stack
    conditional_group.mode.set(print_mode.into());
    state.mode_stack.push(print_mode);
}

pub fn decide_start_tag_kind<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [DOMNode<'s>],
    start_tag: &StartTagKind,
) {
    match start_tag {
        StartTagKind::Group(group) => {
            decide_group(config, state, get_next_nodes, group)
        }
        StartTagKind::ConditionalGroup(conditional_group) => {
            decide_conditional_group(
                config,
                state,
                get_next_nodes,
                conditional_group,
            )
        }
        StartTagKind::Indent => {
            state.states_stack.push(
                state
                    .states_stack
                    .last()
                    .unwrap()
                    .clone()
                    .with_indent(1),
            );
        }
    }
}

pub fn decide_end_tag_kind(state: &mut State, end_tag_kind: EndTagKind) {
    if let Some(last_state) = state.states_stack.pop() {
        assert_eq!(last_state.expected_end_tag_kind, end_tag_kind);
    }
}

pub fn decide_tag<'s: 'n, 'n>(
    config: &Config,
    state: &mut State,
    get_next_nodes: impl Fn() -> &'n [DOMNode<'s>],
    tag: &Tag,
) {
    match tag {
        Tag::Start(start_tag_kind) => decide_start_tag_kind(
            config,
            state,
            get_next_nodes,
            &start_tag_kind,
        ),
        Tag::End(end_tag_kind) => decide_end_tag_kind(state, *end_tag_kind),
    }
}
