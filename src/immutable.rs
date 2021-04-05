use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

use serde_derive::Serialize;
use serde_derive::Deserialize;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct State {
    s: char,
}

impl State {
    pub fn new(s: char) -> State {
        State { s }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Symbol {
    s: char,
}

impl Symbol {
    pub fn new(s: char) -> Symbol {
        Symbol { s }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
}

pub type TransitionFunction = HashMap<(State, Symbol), (State, Symbol, Direction)>;

#[derive(Serialize, Deserialize)]
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

mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;
    use serde;

    use crate::Configuration;
    use crate::Direction;
    use crate::Machine;
    use crate::State;
    use crate::Symbol;

    #[test]
    fn busy_beaver() {

        // Test for the three-state busy beaver Turing machine:
        // https://en.wikipedia.org/wiki/Turing_machine

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

        // Run on the initially empty tape and collect the configurations.
        let mut tape = VecDeque::new();
        tape.push_back(zero);
        let configurations: Vec<Configuration> = machine.iter(tape).collect();

        // Turn the configurations into a string with one
        // configuration per line.
        let tableau: Vec<String> = configurations.iter().map(|x| format!("{}", x)).collect();
        let actual = tableau.join("\n");
        let expected = "
1b0
a11
c011
b0111
a01111
1b1111
11b111
111b11
1111b1
11111b0
1111a11
111c111
1111h11
".trim();
        assert_eq!(actual, expected);
    }

    #[test]
    fn roundtrip() {
        let expected = "
    abch
    01
    0
    1
    a
    a0b1r
    a1c1l
    b0a1l
    b1b1r
    c0b1l
    c1h1r
".trim_start();

        let actual: String = serde_json::from_str(&serde_json::to_string(expected).unwrap()).unwrap();
        assert_eq!(actual, expected);
    }

}
