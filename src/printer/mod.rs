use crate::{
    dom::{
        node::{DOMNode, LineMode},
        tag::{EndTagKind, StartTagKind, Tag},
    },
    shared::PrintMode,
};

pub struct Config {
    pub indent_width: crate::shared::IndentWidth,
    pub print_mode_stack: Vec<crate::shared::PrintMode>,
    pub new_line_control_sequence: &'static [u8],
}

pub struct State {
    pub indent_level: crate::shared::IndentLevel,
    pub is_indent_flushed: bool,
    pub group_level: usize,
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
            group_level: 0,
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
    line_mode: LineMode,
) -> std::io::Result<()> {
    let print_mode = *config.print_mode_stack.get(state.group_level).unwrap();
    match line_mode {
        LineMode::Hard => print_new_line(io_writer, config, state),
        LineMode::Empty => {
            print_new_line(io_writer, config, state)?;
            print_new_line(io_writer, config, state)?;
            Ok(())
        }
        LineMode::Soft => {
            if PrintMode::Expanded == print_mode {
                print_new_line(io_writer, config, state)
            } else {
                Ok(())
            }
        }
        LineMode::SoftOrSpace => match print_mode {
            PrintMode::Flat => io_writer.write_all(&[b' ']),
            PrintMode::Expanded => print_new_line(io_writer, config, state),
        },
    }
}

pub fn process_start_tag_kind(state: &mut State, kind: &StartTagKind) {
    match kind {
        StartTagKind::Indent => {
            state.indent_level += 1;
        }
        StartTagKind::Group(_) => {
            state.group_level += 1;
        }
        StartTagKind::ConditionalGroup(_) => {
            state.group_level += 1;
        }
    }
}

pub fn process_end_tag_kind(state: &mut State, kind: &EndTagKind) {
    match kind {
        EndTagKind::Indent => {
            state.indent_level -= 1;
        }
        EndTagKind::Group => {
            state.group_level -= 1;
        }
        EndTagKind::ConditionalGroup => {
            state.group_level -= 1;
        }
    }
}

pub fn process_tag(state: &mut State, tag: &Tag) {
    match tag {
        Tag::Start(start_tag_kind) => {
            process_start_tag_kind(state, start_tag_kind)
        }
        Tag::End(end_tag_kind) => process_end_tag_kind(state, end_tag_kind),
    }
}

pub fn print_node<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
    node: &DOMNode<'s>,
) -> std::io::Result<()> {
    match node {
        DOMNode::Space => io_writer.write_all(&[b' ']),
        DOMNode::Token(token) => io_writer.write_all(token.as_bytes()),
        DOMNode::Text(text) => io_writer.write_all(text.text.as_bytes()),
        DOMNode::Line(line_mode) => {
            print_line_mode(io_writer, config, state, *line_mode)
        }
        DOMNode::Tag(tag) => {
            process_tag(state, tag);
            Ok(())
        }
    }
}

pub fn print_nodes<'s, IOWriter: std::io::Write>(
    io_writer: &mut IOWriter,
    config: &Config,
    state: &mut State,
    nodes: &[DOMNode<'s>],
) -> std::io::Result<()> {
    nodes
        .iter()
        .map(|node| print_node(io_writer, config, state, node))
        .collect()
}

#[cfg(test)]
mod printer_tests {
    use crate::{
        dom::{
            builders,
            tag::shared::{Group, GroupMode},
        },
        shared::IndentWidth,
    };

    use super::*;

    fn compute_char_width(c: char) -> usize {
        unicode_width::UnicodeWidthChar::width(c).unwrap_or_default()
    }

    #[test]
    fn test_printer() {
        let mut io_writer = Vec::<u8>::new();
        let config = Config {
            print_mode_stack: vec![
                PrintMode::Flat,
                PrintMode::Flat,
                PrintMode::Expanded,
            ],
            new_line_control_sequence: b"\r\n",
            indent_width: IndentWidth::from_u8(4).unwrap(),
        };
        let mut state = State::default();
        print_nodes(
            &mut io_writer,
            &config,
            &mut state,
            &builders::wrap_in_group(
                Group {
                    id: None,
                    mode: std::cell::Cell::new(GroupMode::Flat),
                },
                &[
                    &[
                        builders::token("extend"),
                        builders::soft_line_or_space(),
                        builders::token("query"),
                        builders::soft_line_or_space(),
                        builders::text(
                            "Query",
                            config.indent_width,
                            compute_char_width,
                        ),
                    ],
                    builders::wrap_in_group(
                        Group {
                            id: None,
                            mode: std::cell::Cell::new(GroupMode::Expand),
                        },
                        &[
                            builders::token("{"),
                            builders::soft_line(),
                            builders::token("field"),
                            builders::token(":"),
                            builders::token("Int"),
                            builders::token("!"),
                            builders::soft_line(),
                            builders::token("}"),
                        ],
                    )
                    .as_slice(),
                ]
                .concat(),
            ),
        )
        .unwrap();
        let result = String::from_utf8(io_writer).unwrap();
        print!("{}", result);
        assert!(false);
    }
}
