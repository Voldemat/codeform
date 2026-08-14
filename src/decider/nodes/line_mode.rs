use crate::{
    decider::{config::Config, state::State},
    dom::node::LineMode,
    shared::PrintMode,
};

pub fn decide_line_mode(
    config: &Config,
    state: &mut State,
    print_mode: PrintMode,
    line_mode: LineMode,
) {
    match (line_mode, print_mode) {
        (LineMode::Hard | LineMode::Empty, _)
        | (LineMode::SoftOrSpace | LineMode::Soft, PrintMode::Expanded) => {
            state.reset_line(config.indent_width)
        }
        (LineMode::SoftOrSpace, PrintMode::Flat) => {
            state.advance(1);
        }
        (LineMode::Soft, PrintMode::Flat) => {}
    }
}
