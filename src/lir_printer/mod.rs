use crate::ir::lir;

pub struct Config {
    pub indent_width: crate::ir::shared::IndentWidth,
    pub new_line_control_sequence: &'static [u8],
}

pub struct State {
    pub indent_level: crate::ir::shared::IndentLevel,
    pub is_indent_flushed: bool,
    pub is_empty_line_pending: bool,
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
            is_empty_line_pending: false,
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
            state.is_empty_line_pending = true;
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

pub fn flush_indent_if_needed<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
) -> std::io::Result<()> {
    if state.is_indent_flushed {
        Ok(())
    } else {
        state.is_indent_flushed = true;
        io_writer.write_all(
            " ".repeat(
                state.indent_level as usize
                    * config.indent_width.value() as usize,
            )
            .as_bytes(),
        )
    }
}

pub fn flush_new_line_if_needed<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
) -> std::io::Result<()> {
    if !state.is_empty_line_pending {
        Ok(())
    } else {
        state.is_empty_line_pending = false;
        print_new_line(io_writer, config, state)
    }
}

pub fn print_bytes<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
    value: &[u8],
) -> std::io::Result<()> {
    flush_new_line_if_needed(io_writer, config, state)
        .and_then(|_| flush_indent_if_needed(io_writer, config, state))
        .and_then(|_| io_writer.write_all(value))
}

pub fn print_node<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
    node: &lir::node::Node<'s>,
) -> std::io::Result<()> {
    match node {
        lir::node::Node::Byte(byte) => {
            print_bytes(io_writer, config, state, &[*byte])
        }
        lir::node::Node::Text(ascii_text) => {
            print_bytes(io_writer, config, state, ascii_text.as_bytes())
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
