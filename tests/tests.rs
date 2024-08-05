use gclient::{EventProcessor, GearApi, Result};
use gstd::{prelude::*, ActorId};
use gtest::{Log, Program, System};
use std::fs;
use template_io::*;

#[test]
fn test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);

    let mut result = program.send_bytes(2, []);

    assert!(!result.main_failed());

    // result = program.send(2, PingPong::Pong);

    // assert!(!result.main_failed());

    // State reading

    // All state

    let mut expected_state = vec![];

    {
        let actor = 3;
        result = program.send(actor, PingPong::Ping);

        assert!(result.contains(&Log::builder().payload(PingPong::Pong)));

        expected_state.push((actor.into(), 1))
    }

    system.spend_blocks(101);

    let mut state: Vec<(ActorId, i128)> = program.read_state(b"").unwrap();

    expected_state.sort_unstable();
    state.sort_unstable();

    assert_eq!(state, expected_state);

    // Querying `StateQuery::PingCount` from the `query` metafunction

    result = program.send(2, PingPong::Ping);

    assert!(result.contains(&Log::builder().payload(PingPong::Pong)));
}
