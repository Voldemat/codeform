use crate::{
    dom::{
        node::{DOMNode, LineMode},
        tag::{EndTagKind, StartTagKind, Tag},
    },
    shared::{PrintMode, TextWidth, Width},
};

#[derive(Debug, Clone)]
pub struct MaybeWillFitState {
    pub remaining_width: u32,
    pub group_depth: u32,
}

impl MaybeWillFitState {
    pub fn with_decreased_remaining_width(self: Self, diff_width: u32) -> Self {
        Self {
            remaining_width: self.remaining_width - diff_width,
            group_depth: self.group_depth,
        }
    }

    pub fn with_decremented_group_depth(self: Self) -> Self {
        Self {
            remaining_width: self.remaining_width,
            group_depth: self.group_depth.saturating_sub(1),
        }
    }

    pub fn with_incremented_group_depth(self: Self) -> Self {
        Self {
            remaining_width: self.remaining_width,
            group_depth: self.group_depth.saturating_add(1),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeFitState {
    DefinitelyFits,
    DoesNotFit,
    MaybeWillFit(MaybeWillFitState),
}

impl NodeFitState {
    pub fn to_print_mode(self: &Self) -> PrintMode {
        match self {
            Self::DefinitelyFits | Self::MaybeWillFit(_) => PrintMode::Flat,
            Self::DoesNotFit => PrintMode::Expanded,
        }
    }
}

pub fn does_space_fit(fit_state: MaybeWillFitState) -> NodeFitState {
    if fit_state.remaining_width < 1 {
        NodeFitState::DoesNotFit
    } else {
        NodeFitState::MaybeWillFit(fit_state.with_decreased_remaining_width(1))
    }
}

pub fn does_width_fit(
    fit_state: MaybeWillFitState,
    width: Width,
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
    text_width: TextWidth,
) -> NodeFitState {
    match text_width {
        TextWidth::Width(w) => does_width_fit(fit_state, w),
        TextWidth::Multiline => NodeFitState::DoesNotFit,
    }
}

pub fn does_line_mode_fit(
    fit_state: MaybeWillFitState,
    line_mode: LineMode,
) -> NodeFitState {
    match line_mode {
        LineMode::Hard | LineMode::Empty => NodeFitState::DefinitelyFits,
        LineMode::SoftOrSpace => {
            if fit_state.remaining_width < 1 {
                NodeFitState::DoesNotFit
            } else {
                NodeFitState::MaybeWillFit(
                    fit_state.with_decreased_remaining_width(1),
                )
            }
        }
        LineMode::Soft => NodeFitState::MaybeWillFit(fit_state),
    }
}

pub fn process_start_tag(
    fit_state: MaybeWillFitState,
    start_tag: &StartTagKind,
) -> NodeFitState {
    match start_tag {
        StartTagKind::Group(_) | StartTagKind::ConditionalGroup(_) => {
            NodeFitState::MaybeWillFit(fit_state.with_incremented_group_depth())
        }
        _ => NodeFitState::MaybeWillFit(fit_state),
    }
}

pub fn process_end_tag_kind(
    fit_state: MaybeWillFitState,
    end_tag_kind: &EndTagKind,
) -> NodeFitState {
    match end_tag_kind {
        EndTagKind::Group | EndTagKind::ConditionalGroup => {
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

pub fn process_tag(fit_state: MaybeWillFitState, tag: &Tag) -> NodeFitState {
    match tag {
        Tag::Start(start_tag) => process_start_tag(fit_state, start_tag),
        Tag::End(end_tag_kind) => process_end_tag_kind(fit_state, end_tag_kind),
    }
}

pub fn does_token_fit<'s>(
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

pub fn does_node_fit<'s>(
    fit_state: MaybeWillFitState,
    node: &DOMNode<'s>,
) -> NodeFitState {
    match node {
        DOMNode::Space => does_space_fit(fit_state),
        DOMNode::Text(text) => does_text_width_fit(fit_state, text.width),
        DOMNode::Token(token) => does_token_fit(fit_state, token),
        DOMNode::Line(line_mode) => does_line_mode_fit(fit_state, *line_mode),
        DOMNode::Tag(tag) => process_tag(fit_state, tag),
    }
}

pub fn do_nodes_fit<'s>(
    fit_state: MaybeWillFitState,
    nodes: &[DOMNode<'s>],
) -> NodeFitState {
    nodes.iter().fold(
        NodeFitState::MaybeWillFit(fit_state),
        |current_fit_state, child_node| match current_fit_state {
            NodeFitState::DoesNotFit => current_fit_state,
            NodeFitState::DefinitelyFits => current_fit_state,
            NodeFitState::MaybeWillFit(maybe_will_fit_state) => {
                does_node_fit(maybe_will_fit_state, child_node)
            }
        },
    )
}
