#[cfg(test)]
mod group {
    use std::cell::Cell;

    use crate::{
        decider::{config::Config, nodes::nodes::decide_nodes, state::State},
        dom::{
            builders, getters,
            tag::shared::{Group, GroupMode},
        },
        shared::{IndentWidth, LineWidth, PrintMode},
    };

    fn create_config() -> Config {
        Config {
            max_width: LineWidth::new(80).unwrap(),
            indent_width: IndentWidth::from_u8(4).unwrap(),
        }
    }

    #[test]
    fn test_group_stays_flat_if_fits() {
        let config = create_config();
        let mut state = State::default();

        let nodes = builders::wrap_in_group(
            Group {
                id: None,
                mode: Cell::new(GroupMode::Flat),
            },
            &[
                builders::tokens(&["extend", "type", "Query", "{"]).as_slice(),
                &[builders::soft_line()],
                &builders::tokens(&["field", ":", "Int", "!"]),
                &[builders::soft_line(), builders::token("}")],
            ]
            .concat(),
        );
        decide_nodes(&config, &mut state, &nodes);
        pretty_assertions::assert_eq!(state.mode_stack, vec![PrintMode::Flat]);
        pretty_assertions::assert_eq!(
            getters::node_as_group(nodes.first().unwrap()).mode.get(),
            GroupMode::Flat
        );
    }

    #[test]
    fn test_group_expands_if_does_not_fit() {
        let config = create_config();
        let mut state = State::default();

        let nodes = builders::wrap_in_group(
            Group {
                id: None,
                mode: Cell::new(GroupMode::Flat),
            },
            &[
                builders::tokens(&["extend", "type", "Query", "{"]).as_slice(),
                &[builders::soft_line()],
                &builders::tokens(&["fieldasbdasbdasjkdbajslbdajbdajbdjkasbdjkasbdjabdjkasasdsahdasdnasdsakldanklsd", ":", "Int", "!"]),
                &[builders::soft_line(), builders::token("}")],
            ]
            .concat(),
        );
        decide_nodes(&config, &mut state, &nodes);
        pretty_assertions::assert_eq!(
            state.mode_stack,
            vec![PrintMode::Expanded]
        );
        pretty_assertions::assert_eq!(
            getters::node_as_group(nodes.first().unwrap()).mode.get(),
            GroupMode::Expand
        );
    }
}
