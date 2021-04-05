use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct State {
    s: char,
}

impl State {
    pub fn new(s: char) -> State {
        State { s }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Symbol {
    s: char,
}

impl Symbol {
    pub fn new(s: char) -> Symbol {
        Symbol { s }
    }
}

pub enum Direction {
    Left,
    Right,
}

pub type TransitionFunction = HashMap<(State, Symbol), (State, Symbol, Direction)>;

pub struct Machine {
    states: HashSet<State>,                  // Q
    tape_alphabet: HashSet<Symbol>,          // Gamma
    blank_symbol: Symbol,                    // b
    input_alphabet: HashSet<Symbol>,         // Sigma
    initial_state: State,                    // q_0
    accepting_states: HashSet<State>,        // accepting states
    transition_function: TransitionFunction, // delta
}

impl Machine {
    pub fn new(
        states: HashSet<State>,
        tape_alphabet: HashSet<Symbol>,
        blank_symbol: Symbol,
        input_alphabet: HashSet<Symbol>,
        initial_state: State,
        accepting_states: HashSet<State>,
        transition_function: TransitionFunction,
    ) -> Machine {
        Machine {
            states,
            tape_alphabet,
            blank_symbol,
            input_alphabet,
            initial_state,
            accepting_states,
            transition_function,
        }
    }

    pub fn iter(&self, tape: VecDeque<Symbol>) -> ConfigurationIterator {
        ConfigurationIterator {
            transition_function: &self.transition_function,
            configuration: Configuration {
                state: self.initial_state,
                head: 0,
                tape: tape,
            },
            blank_symbol: self.blank_symbol,
        }
    }
}

pub struct Configuration {
    state: State,
    head: usize,
    tape: VecDeque<Symbol>,
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Get the symbols from the tape.
        let (prefix, suffix) = self.tape.as_slices();
        let symbols = [prefix, suffix].concat();

        // Split the symbols at the head location.
        let (left, right) = &symbols.split_at(self.head);

        // Insert the state at the head location.
        let mut result = String::new();
        for symbol in left.iter() {
            result.push(symbol.s);
        }
        result.push(self.state.s);
        for symbol in right.iter() {
            result.push(symbol.s);
        }
        write!(f, "{}", result)
    }
}

pub struct ConfigurationIterator<'a> {
    transition_function: &'a TransitionFunction,
    configuration: Configuration,
    blank_symbol: Symbol,
}

impl<'a> Iterator for ConfigurationIterator<'a> {
    type Item = Configuration;

    fn next(&mut self) -> Option<Self::Item> {
        let Configuration { state, head, tape } = &self.configuration;

        // Read the current symbol.
        let symbol = tape[*head];

        // Apply the transition function based on the current state
        // and the current symbol.
        //
        // If the transition function does not have an entry, then we
        // assume that means the current state is a halting state, so
        // we terminate the iterator by returning `None`.
        let input = (*state, symbol);
        let output = match self.transition_function.get(&input) {
            Some(o) => o,
            None => {
                return None;
            }
        };
        let (next_state, write_symbol, direction) = output;

        // Write the symbol.
        let mut next_tape = tape.clone();
        next_tape[*head] = *write_symbol;

        // Move the head left or right, extending the `VecDeque`
        // representing the tape data if necessary.
        let next_head = match direction {
            Direction::Right => {
                if *head == next_tape.len() - 1 {
                    next_tape.push_back(self.blank_symbol);
                }
                head + 1
            }
            Direction::Left => {
                if *head == 0 {
                    next_tape.push_front(self.blank_symbol);
                    0
                } else {
                    head - 1
                }
            }
        };

        let configuration_to_store = Configuration {
            state: *next_state,
            head: next_head,
            tape: next_tape.clone(),
        };
        self.configuration = configuration_to_store;

        let configuration_to_return = Configuration {
            state: *next_state,
            head: next_head,
            tape: next_tape,
        };

        Some(configuration_to_return)
    }
}

// fn main() {
// }
