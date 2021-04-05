use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use turingrs::Direction;
use turingrs::Machine;
use turingrs::State;
use turingrs::Symbol;

fn main() {
    // States
    let a = State::new('a');
    let b = State::new('b');
    let c = State::new('c');
    let halt = State::new('h');
    let mut states = HashSet::new();
    states.insert(a);
    states.insert(b);
    states.insert(c);
    states.insert(halt);

    // Tape alphabet
    let zero = Symbol::new('0');
    let one = Symbol::new('1');
    let mut tape_alphabet = HashSet::new();
    tape_alphabet.insert(zero);
    tape_alphabet.insert(one);

    // Blank symbol
    let blank_symbol = zero;

    // Input symbols
    let mut input_alphabet = HashSet::new();
    input_alphabet.insert(one);

    // Initial state
    let initial_state = a;

    // Accepting states
    let mut accepting_states = HashSet::new();
    accepting_states.insert(halt);

    // Transition function
    let mut transition_function = HashMap::new();
    transition_function.insert((a, zero), (b, one, Direction::Right));
    transition_function.insert((a, one), (c, one, Direction::Left));
    transition_function.insert((b, zero), (a, one, Direction::Left));
    transition_function.insert((b, one), (b, one, Direction::Right));
    transition_function.insert((c, zero), (b, one, Direction::Left));
    transition_function.insert((c, one), (halt, one, Direction::Right));

    // Machine
    let machine = Machine::new(
        states,
        tape_alphabet,
        blank_symbol,
        input_alphabet,
        initial_state,
        accepting_states,
        transition_function,
    );

    // Run on the initially empty tape.
    let mut tape = VecDeque::new();
    tape.push_back(zero);
    for configuration in machine.iter(tape) {
        println!("{}", configuration);
    }
}
