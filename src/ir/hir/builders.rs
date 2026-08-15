use crate::ir::shared::IndentWidth;

use super::{
    node::{LineMode, Node},
    tag::{
        EndTagKind, StartTagKind, Tag,
        shared::{ConditionalGroup, Group},
    },
    text::Text,
    text_width::TextWidth,
};

pub fn unicode_text<F: Fn(char) -> usize>(
    text: &str,
    indent_width: IndentWidth,
    compute_char_width: F,
) -> Node<'_> {
    Node::Text(Text {
        text,
        width: TextWidth::from_text(text, indent_width, compute_char_width),
    })
}

pub fn ascii_text(value: &str) -> Node<'_> {
    Node::AsciiOnelineText(value)
}

pub fn soft_line<'s>() -> Node<'s> {
    Node::Line(LineMode::Soft)
}

pub fn hard_line<'s>() -> Node<'s> {
    Node::Line(LineMode::Hard)
}

pub fn soft_line_or_space<'s>() -> Node<'s> {
    Node::Line(LineMode::SoftOrSpace)
}

pub fn start_group<'s>(group: Group) -> Node<'s> {
    Node::Tag(Tag::Start(StartTagKind::Group(group)))
}

pub fn end_group<'s>() -> Node<'s> {
    Node::Tag(Tag::End(EndTagKind::Group))
}

pub fn wrap_in_group<'s>(group: Group, nodes: &[Node<'s>]) -> Vec<Node<'s>> {
    let mut new_nodes = vec![start_group(group)];
    new_nodes.extend_from_slice(nodes);
    new_nodes.push(end_group());
    new_nodes
}

pub fn start_conditional_group<'s>(
    conditional_group: ConditionalGroup,
) -> Node<'s> {
    Node::Tag(Tag::Start(StartTagKind::ConditionalGroup(
        conditional_group,
    )))
}

pub fn end_conditional_group<'s>() -> Node<'s> {
    Node::Tag(Tag::End(EndTagKind::ConditionalGroup))
}

pub fn wrap_in_conditional_group<'s>(
    conditional_group: ConditionalGroup,
    nodes: Vec<Node<'s>>,
) -> Vec<Node<'s>> {
    let mut new_nodes = vec![start_conditional_group(conditional_group)];
    new_nodes.extend(nodes);
    new_nodes.push(end_conditional_group());
    new_nodes
}
