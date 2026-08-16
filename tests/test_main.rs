use codeform::ir::hir;
use codeform::ir::shared;

#[test]
fn test_main() {
    let hir_nodes = [
        &[
            hir::builders::ascii_oneline_text("extend"),
            hir::builders::byte(b' '),
            hir::builders::ascii_oneline_text("type"),
            hir::builders::byte(b' '),
            hir::builders::ascii_oneline_text("Query"),
            hir::builders::byte(b' '),
        ],
        hir::builders::wrap_in_group(
            hir::tag::shared::Group {
                default_print_mode: shared::PrintMode::Flat,
                id: None,
            },
            &[
                &[hir::builders::byte(b'{'), hir::builders::soft_line()],
                hir::builders::wrap_in_indent(
                    hir::tag::IndentMode::Soft,
                    &[hir::builders::ascii_oneline_text("asdnqkdalkdalksdnlkasndklasndlkasndlkasndbklasndklasndkalndklandakdnakldnaklsndlkasnasndasndaskdnakldnaslkdnlkd")],
                )
                .as_slice(),
                &[hir::builders::soft_line(), hir::builders::byte(b'}')],
            ]
            .concat(),
        )
        .as_slice(),
    ]
    .concat();
    let indent_width = codeform::ir::shared::IndentWidth::from_u8(4).unwrap();
    let config = codeform::hir_to_lir::config::Config {
        max_width: codeform::ir::shared::LineWidth::try_from(80).unwrap(),
        indent_width
    };
    let mut state = codeform::hir_to_lir::state::State::default();
    let lir_nodes = codeform::hir_to_lir::mappers::nodes::lower(
        &config,
        &mut state,
        hir_nodes
    );
    let mut io_writer = Vec::<u8>::new();
    let mut printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        &mut io_writer,
        &codeform::lir_printer::Config {
            indent_width,
            new_line_control_sequence: b"\n"
        },
        &mut printer_state,
        &lir_nodes
    ).unwrap();
    let final_result = String::from_utf8(io_writer).unwrap();
    pretty_assertions::assert_eq!(final_result, r#"extend type Query {
    asdnqkdalkdalksdnlkasndklasndlkasndlkasndbklasndklasndkalndklandakdnakldnaklsndlkasnasndasndaskdnakldnaslkdnlkd
}"#);
}
