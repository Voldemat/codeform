#[cfg(test)]
mod conditional_group {
    use std::cell::Cell;

    use crate::{
        decider::{config::Config, nodes::nodes::decide_nodes, state::State},
        dom::{
            builders, getters,
            tag::shared::{Condition, ConditionalGroup, GroupMode},
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
    fn test_conditional_group_stays_flat_when_condition_matches_and_fits() {
        let config = create_config();
        let mut state = State::default();
        let target_id = std::num::NonZeroU32::new(1).unwrap();

        // 1. Simulate that the target group resolved to Expanded
        state.group_mode_map.insert(target_id, PrintMode::Expanded);

        // 2. ConditionalGroup expects target_id to be Expanded
        let conditional_group = ConditionalGroup {
            mode: Cell::new(GroupMode::Flat),
            condition: Condition {
                expected_mode: PrintMode::Expanded,
                target_group_id: target_id,
            },
        };

        let nodes = builders::wrap_in_conditional_group(
            conditional_group,
            [
                builders::tokens(&["extend", "type", "Query", "{"]).as_slice(),
                &[builders::soft_line()],
                &builders::tokens(&["field", ":", "Int", "!"]),
                &[builders::soft_line(), builders::token("}")],
            ]
            .concat(),
        );

        decide_nodes(&config, &mut state, &nodes);

        pretty_assertions::assert_eq!(
            getters::node_as_conditional_group(nodes.first().unwrap())
                .mode
                .get(),
            GroupMode::Flat
        );
    }

    #[test]
    fn test_conditional_group_expands_when_condition_matches_and_does_not_fit()
    {
        let config = create_config();
        let mut state = State::default();
        let target_id = std::num::NonZeroU32::new(1).unwrap();

        state.group_mode_map.insert(target_id, PrintMode::Expanded);

        let conditional_group = ConditionalGroup {
            mode: Cell::new(GroupMode::Flat),
            condition: Condition {
                expected_mode: PrintMode::Expanded,
                target_group_id: target_id,
            },
        };

        let nodes = builders::wrap_in_conditional_group(
            conditional_group,
            [
                builders::tokens(&["extend", "type", "Query", "{"]).as_slice(),
                &[builders::soft_line()],
                &builders::tokens(&[
                    "fieldasbdasbdasjkdbajslbdajbdajbdjkasbdjkasbdjabdjkasasdsahdasdnasdsakldanklsd",
                    ":",
                    "Int",
                    "!",
                ]),
                &[builders::soft_line(), builders::token("}")],
            ]
            .concat(),
        );

        decide_nodes(&config, &mut state, &nodes);

        pretty_assertions::assert_eq!(
            getters::node_as_conditional_group(nodes.first().unwrap())
                .mode
                .get(),
            GroupMode::Expand
        );
    }

    #[test]
    fn test_conditional_group_skips_flat_evaluation_when_condition_mismatches()
    {
        let config = create_config();
        let mut state = State::default();
        let target_id = std::num::NonZeroU32::new(1).unwrap();

        // Target group resolved to Flat in state
        state.group_mode_map.insert(target_id, PrintMode::Flat);

        // ConditionalGroup expected Target Group to be Expanded
        let conditional_group = ConditionalGroup {
            mode: Cell::new(GroupMode::Flat),
            condition: Condition {
                expected_mode: PrintMode::Expanded,
                target_group_id: target_id,
            },
        };

        // Short content that WOULD fit flat if evaluated
        let nodes = builders::wrap_in_conditional_group(
            conditional_group,
            builders::tokens(&["short", "content"]).to_vec(),
        );

        decide_nodes(&config, &mut state, &nodes);

        // Since the condition was not met (Flat != Expanded),
        // the group defaults to Expanded without attempting to format flat.
        pretty_assertions::assert_eq!(
            getters::node_as_conditional_group(nodes.first().unwrap())
                .mode
                .get(),
            GroupMode::Expand
        );
    }
}
