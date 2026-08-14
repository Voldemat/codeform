use crate::decider::state::State;

pub fn decide_token(state: &mut State, token: &str) {
    state.advance(token.len() as u32);
}
