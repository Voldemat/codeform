use crate::shared::{IndentWidth, TextWidth};

use super::{
    node::{DOMNode, LineMode, Text},
    tag::{
        EndTagKind, StartTagKind, Tag,
        shared::{ConditionalGroup, Group},
    },
};

pub fn text<F: Fn(char) -> usize>(
    text: &str,
    indent_width: IndentWidth,
    compute_char_width: F,
) -> DOMNode<'_> {
    DOMNode::Text(Text {
        text,
        width: TextWidth::from_text(text, indent_width, compute_char_width),
    })
}

pub fn token(value: &str) -> DOMNode<'_> {
    DOMNode::Token(value)
}

pub fn tokens<'s>(values: &[&'s str]) -> Vec<DOMNode<'s>> {
    values.iter().map(|value| token(value)).collect()
}

pub fn soft_line<'s>() -> DOMNode<'s> {
    DOMNode::Line(LineMode::Soft)
}

pub fn hard_line<'s>() -> DOMNode<'s> {
    DOMNode::Line(LineMode::Hard)
}

pub fn soft_line_or_space<'s>() -> DOMNode<'s> {
    DOMNode::Line(LineMode::SoftOrSpace)
}

pub fn start_group<'s>(group: Group) -> DOMNode<'s> {
    DOMNode::Tag(Tag::Start(StartTagKind::Group(group)))
}

pub fn end_group<'s>() -> DOMNode<'s> {
    DOMNode::Tag(Tag::End(EndTagKind::Group))
}

pub fn wrap_in_group<'s>(
    group: Group,
    nodes: &[DOMNode<'s>],
) -> Vec<DOMNode<'s>> {
    let mut new_nodes = vec![start_group(group)];
    new_nodes.extend_from_slice(nodes);
    new_nodes.push(end_group());
    new_nodes
}

pub fn start_conditional_group<'s>(
    conditional_group: ConditionalGroup,
) -> DOMNode<'s> {
    DOMNode::Tag(Tag::Start(StartTagKind::ConditionalGroup(
        conditional_group,
    )))
}

pub fn end_conditional_group<'s>() -> DOMNode<'s> {
    DOMNode::Tag(Tag::End(EndTagKind::ConditionalGroup))
}

pub fn wrap_in_conditional_group<'s>(
    conditional_group: ConditionalGroup,
    nodes: Vec<DOMNode<'s>>,
) -> Vec<DOMNode<'s>> {
    let mut new_nodes = vec![start_conditional_group(conditional_group)];
    new_nodes.extend(nodes);
    new_nodes.push(end_conditional_group());
    new_nodes
}
