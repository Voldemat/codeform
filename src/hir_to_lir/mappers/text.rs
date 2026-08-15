use crate::{
    hir_to_lir::{config::Config, state::State},
    ir::{hir, lir},
};

pub fn process_text_width(
    config: &Config,
    state: &mut State,
    text_width: hir::TextWidth,
) {
    match text_width {
        hir::TextWidth::Width(width) => {
            state.advance(width.value());
        }
        hir::TextWidth::Multiline => {
            state.reset_line(config.indent_width);
        }
    }
}

pub fn lower_ascii_text<'s>(
    state: &mut State,
    ascii_text: &'s str,
) -> lir::node::Node<'s> {
    state.advance(ascii_text.len() as u32);
    lir::node::Node::Text(ascii_text)
}

pub fn lower_unicode_text<'s>(
    config: &Config,
    state: &mut State,
    unicode_text: hir::Text<'s>,
) -> lir::node::Node<'s> {
    process_text_width(config, state, unicode_text.width);
    lir::node::Node::Text(unicode_text.text)
}
