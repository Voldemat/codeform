use crate::ir::lir;

pub struct Config {
    pub indent_width: crate::ir::shared::IndentWidth,
    pub new_line_control_sequence: &'static [u8],
}

pub struct State {
    pub indent_level: crate::ir::shared::IndentLevel,
    pub is_indent_flushed: bool,
}

impl State {
    pub fn reset_line(self: &mut Self) {
        self.is_indent_flushed = false;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            indent_level: 0,
            is_indent_flushed: false,
        }
    }
}

pub fn print_new_line<IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
) -> std::io::Result<()> {
    io_writer.write_all(config.new_line_control_sequence)?;
    state.reset_line();
    Ok(())
}

pub fn print_line_mode<IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
    line_mode: lir::node::LineMode,
) -> std::io::Result<()> {
    match line_mode {
        lir::node::LineMode::Normal => print_new_line(io_writer, config, state),
        lir::node::LineMode::Empty => {
            print_new_line(io_writer, config, state)?;
            print_new_line(io_writer, config, state)?;
            Ok(())
        }
    }
}

pub fn process_start_tag_kind(
    state: &mut State,
    kind: &lir::tag::StartTagKind,
) {
    match kind {
        lir::tag::StartTagKind::Indent => {
            state.indent_level += 1;
        }
    }
}

pub fn process_end_tag_kind(state: &mut State, kind: &lir::tag::EndTagKind) {
    match kind {
        lir::tag::EndTagKind::Indent => {
            state.indent_level -= 1;
        }
    }
}

pub fn process_tag(state: &mut State, tag: &lir::tag::Tag) {
    match tag {
        lir::tag::Tag::Start(start_tag_kind) => {
            process_start_tag_kind(state, start_tag_kind)
        }
        lir::tag::Tag::End(end_tag_kind) => {
            process_end_tag_kind(state, end_tag_kind)
        }
    }
}

pub fn print_node<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
    node: &lir::node::Node<'s>,
) -> std::io::Result<()> {
    match node {
        lir::node::Node::Byte(byte) => io_writer.write_all(&[*byte]),
        lir::node::Node::Text(ascii_text) => {
            io_writer.write_all(ascii_text.as_bytes())
        }
        lir::node::Node::Line(line_mode) => {
            print_line_mode(io_writer, config, state, *line_mode)
        }
        lir::node::Node::Tag(tag) => {
            process_tag(state, tag);
            Ok(())
        }
    }
}

pub fn print_nodes<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
    nodes: &[lir::node::Node<'s>],
) -> std::io::Result<()> {
    nodes
        .iter()
        .map(|node| print_node(io_writer, config, state, node))
        .collect()
}
