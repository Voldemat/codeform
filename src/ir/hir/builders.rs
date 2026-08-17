use crate::ir::shared::{IndentWidth, PrintMode};

use super::{
    node::{LineMode, Node},
    tag::{
        ConditionalGroup, EndTagKind, Group, GroupId, IndentMode, StartTagKind,
        Tag,
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

pub fn ascii_oneline_text(value: &str) -> Node<'_> {
    Node::AsciiOnelineText(value)
}

pub fn soft_line<'s>() -> Node<'s> {
    Node::Line(LineMode::Soft)
}

pub fn hard_line<'s>() -> Node<'s> {
    Node::Line(LineMode::Hard)
}

pub fn space<'s>() -> Node<'s> {
    Node::Byte(b' ')
}

pub fn empty_line<'s>() -> Node<'s> {
    Node::Line(LineMode::Empty)
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

pub fn wrap_in_group<'s, TNodes: IntoIterator<Item = Node<'s>>>(
    group: Group,
    nodes: TNodes,
) -> NodesVec<'s> {
    NodesVec::from_node(start_group(group))
        .extend(nodes)
        .push(end_group())
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

pub fn wrap_in_conditional_group<'s, TNodes: IntoIterator<Item = Node<'s>>>(
    conditional_group: ConditionalGroup,
    nodes: TNodes,
) -> NodesVec<'s> {
    NodesVec::from_node(start_conditional_group(conditional_group))
        .extend(nodes)
        .push(end_conditional_group())
}

pub fn byte<'s>(b: u8) -> Node<'s> {
    Node::Byte(b)
}

pub fn start_indent<'s>(indent_mode: IndentMode) -> Node<'s> {
    Node::Tag(Tag::Start(StartTagKind::Indent(indent_mode)))
}

pub fn end_indent<'s>() -> Node<'s> {
    Node::Tag(Tag::End(EndTagKind::Indent))
}

pub fn wrap_in_indent<'s, TNodes: IntoIterator<Item = Node<'s>>>(
    indent_mode: IndentMode,
    nodes: TNodes,
) -> NodesVec<'s> {
    NodesVec::from_node(start_indent(indent_mode))
        .extend(nodes)
        .push(end_indent())
}

pub fn wrap_in_soft_indent<'s, TNodes: IntoIterator<Item = Node<'s>>>(
    nodes: TNodes,
) -> NodesVec<'s> {
    wrap_in_indent(IndentMode::Soft, nodes)
}

pub fn wrap_in_hard_indent<'s, TNodes: IntoIterator<Item = Node<'s>>>(
    nodes: TNodes,
) -> NodesVec<'s> {
    wrap_in_indent(IndentMode::Hard, nodes)
}

pub fn wrap_in_group_with_indent<'s, TNodes: IntoIterator<Item = Node<'s>>>(
    group: Group,
    indent_mode: IndentMode,
    nodes: TNodes,
) -> NodesVec<'s> {
    wrap_in_group(group, wrap_in_indent(indent_mode, nodes))
}

pub fn wrap_in_group_with_soft_indent<
    's,
    Nodes: IntoIterator<Item = Node<'s>>,
>(
    group: Group,
    nodes: Nodes,
) -> NodesVec<'s> {
    wrap_in_group(group, wrap_in_indent(IndentMode::Soft, nodes))
}

pub fn group(id: Option<GroupId>, default_print_mode: PrintMode) -> Group {
    Group {
        id,
        default_print_mode,
    }
}

pub fn unanonymous_group(default_print_mode: PrintMode) -> Group {
    Group {
        id: None,
        default_print_mode,
    }
}

pub fn unanonymous_default_flat_group() -> Group {
    Group {
        id: None,
        default_print_mode: PrintMode::Flat,
    }
}

pub fn unanonymous_default_expanded_group() -> Group {
    Group {
        id: None,
        default_print_mode: PrintMode::Expanded,
    }
}

pub struct NodesVec<'s>(Vec<Node<'s>>);

impl<'s> NodesVec<'s> {
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn from_node(node: Node<'s>) -> Self {
        Self(vec![node])
    }

    pub fn from_iterator<I: IntoIterator<Item = Node<'s>>>(nodes: I) -> Self {
        Self(nodes.into_iter().collect())
    }

    pub fn push(mut self: Self, node: Node<'s>) -> Self {
        self.0.push(node);
        self
    }

    pub fn push_if(self: Self, predicate: bool, node: Node<'s>) -> Self {
        if predicate { self.push(node) } else { self }
    }

    pub fn extend<I: IntoIterator<Item = Node<'s>>>(
        mut self: Self,
        nodes: I,
    ) -> Self {
        self.0.extend(nodes);
        self
    }

    pub fn extend_if<I: IntoIterator<Item = Node<'s>>>(
        self: Self,
        predicate: bool,
        nodes: I,
    ) -> Self {
        if predicate { self.extend(nodes) } else { self }
    }

    pub fn extend_if_some<
        T,
        I: IntoIterator<Item = Node<'s>>,
        F: Fn(T) -> I,
    >(
        self: Self,
        optional_value: Option<T>,
        callback: F,
    ) -> Self {
        if let Some(value) = optional_value {
            self.extend(callback(value))
        } else {
            self
        }
    }

    pub fn to_vec(self: Self) -> Vec<Node<'s>> {
        self.0
    }

    pub fn as_slice(self: &Self) -> &[Node<'s>] {
        self.0.as_slice()
    }
}

impl<'s> IntoIterator for NodesVec<'s> {
    type Item = Node<'s>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'s> FromIterator<Node<'s>> for NodesVec<'s> {
    fn from_iter<T: IntoIterator<Item = Node<'s>>>(iter: T) -> Self {
        Self::from_iterator(iter)
    }
}

impl<'s> AsRef<[Node<'s>]> for NodesVec<'s> {
    fn as_ref(&self) -> &[Node<'s>] {
        self.0.as_ref()
    }
}
