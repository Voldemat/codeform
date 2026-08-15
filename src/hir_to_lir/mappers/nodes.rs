use crate::{
    hir_to_lir::{config::Config, state::State},
    ir::{hir, lir},
};

pub fn lower<
    's,
    Nodes: IntoIterator<
            IntoIter = impl AsRef<[hir::node::Node<'s>]>
                       + Iterator<Item = hir::node::Node<'s>>,
            Item = hir::node::Node<'s>,
        >,
>(
    config: &Config,
    state: &mut State,
    nodes: Nodes,
) -> Vec<lir::node::Node<'s>> {
    let mut lowered_nodes = Vec::<lir::node::Node<'s>>::new();
    let mut iter = nodes.into_iter();
    while let Some(node) = iter.next() {
        let next_nodes = iter.as_ref();
        if let Some(lowered_node) =
            super::node::lower(config, state, || next_nodes, node)
        {
            lowered_nodes.push(lowered_node)
        }
    }
    lowered_nodes
}
