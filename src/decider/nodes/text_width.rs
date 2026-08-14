use crate::{decider::{config::Config, state::State}, shared::TextWidth};

pub fn decide_text_width(
    config: &Config,
    state: &mut State,
    text_width: TextWidth,
) {
    match text_width {
        TextWidth::Width(width) => {
            state.advance(width.value());
        }
        TextWidth::Multiline => {
            state.reset_line(config.indent_width);
        }
    }
}

