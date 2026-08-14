use super::{
    node::DOMNode,
    tag::shared::{ConditionalGroup, Group},
};

pub fn node_as_group<'n>(node: &'n DOMNode<'_>) -> &'n Group {
    node.as_tag()
        .unwrap()
        .as_start()
        .unwrap()
        .as_group()
        .unwrap()
}

pub fn node_as_conditional_group<'n>(
    node: &'n DOMNode<'_>,
) -> &'n ConditionalGroup {
    node.as_tag()
        .unwrap()
        .as_start()
        .unwrap()
        .as_conditional_group()
        .unwrap()
}
