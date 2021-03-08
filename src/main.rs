use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fmt;
// use std::ops::Index;
// use std::ops::IndexMut;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    s: char,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Symbol {
    s: char,
}

enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Tape {
    data: VecDeque<Symbol>,
    head: usize,
    blank_symbol: Symbol,
}

impl Tape {
    fn new(data: VecDeque<Symbol>, blank_symbol: Symbol) -> Tape {
        Tape {
            data: data,
            head: 0,
            blank_symbol: blank_symbol,
        }
    }

    // fn _extend_to_index(&mut self, index: i32) -> usize {
    //     if index < self.left {
    //         let n = (self.left - index).try_into().unwrap();
    //         let mut blanks = vec![self.blank_symbol; n];
    //         prepend(&mut blanks, &mut self.data);
    //         self.left = index;
    //         0
    //     } else {
    //         if self.right() <= index {
    //             let n = ((index - self.right()) + 1).try_into().unwrap();
    //             let mut blanks = VecDeque::from(vec![self.blank_symbol; n]);
    //             self.data.append(&mut blanks);
    //             index.try_into().unwrap()
    //         } else {
    //             index.try_into().unwrap()
    //         }
    //     }
    // }

    // fn right(&self) -> i32 {
    //     self.left + (self.data.len() as i32)
    // }

    // fn get(&mut self, index: i32) -> &Symbol {
    //     let i = self._extend_to_index(index);
    //     &self.data[i]
    // }

    // fn set(&mut self, index: i32, value: Symbol) {
    //     let i = self._extend_to_index(index);
    //     self.data[i] = value;
    // }

    fn read(&self) -> Symbol {
        self.data[self.head]
    }

    fn write(&mut self, symbol: Symbol) {
        self.data[self.head] = symbol;
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => {
                self.head += 1;
                if self.head == self.data.len() {
                    self.data.push_back(self.blank_symbol);
                }
            }
            Direction::Left => {
                if self.head == 0 {
                    self.data.push_front(self.blank_symbol);
                } else {
                    self.head -= 1;
                }
            }
        }
    }
}

fn prepend(left: &mut Vec<Symbol>, right: &mut VecDeque<Symbol>) {
    loop {
        if let Some(symbol) = left.pop() {
            right.push_front(symbol)
        }
    }
}

// impl Index<i32> for Tape {
//     type Output = Symbol;

//     fn index(&self, index: i32) -> &Self::Output {
//         if index < self.left {
//             let n = (self.left - index).try_into().unwrap();
//             let mut blanks = vec![self.blank_symbol; n];
//             prepend(&mut blanks, &mut self.data);
//             self.left = index;
//         }
//         let i = (index - self.left).try_into().unwrap();
//         &self.data[i]
//     }
// }

// impl IndexMut<i32> for Tape {
//     fn index_mut(&mut self, index: i32) -> &mut Self::Output {
//         if index < self.left {
//             let n = (self.left - index).try_into().unwrap();
//             let mut blanks = vec![self.blank_symbol; n];
//             prepend(&mut blanks, &mut self.data);
//             self.left = index;
//         }
//         let i = (index - self.left).try_into().unwrap();
//         &mut self.data[i]
//     }
// }

struct Machine {
    states: HashSet<State>,                                                    // Q
    tape_alphabet: HashSet<Symbol>,                                            // Gamma
    blank_symbol: Symbol,                                                      // b
    input_alphabet: HashSet<Symbol>,                                           // Sigma
    initial_state: State,                                                      // q_0
    accepting_states: HashSet<State>,                                          // accepting states
    transition_function: HashMap<(State, Symbol), (State, Symbol, Direction)>, // delta
}

struct Configuration<'a> {
    state: State,
    tape: &'a Tape,
}

impl fmt::Display for Configuration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get the symbols from the tape.
        let (prefix, suffix) = self.tape.data.as_slices();
        let symbols = [prefix, suffix].concat();

        // Split the symbols at the head location.
        let (left, right) = &symbols.split_at(self.tape.head);

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

struct RuntimeError {
    message: String,
    // configuration: Configuration,
}

impl Machine {
    fn run(&self, mut tape: Tape) -> Result<bool, RuntimeError> {
        // Define the initial configuration of the machine.
        let mut current_state = self.initial_state;

        loop {
            let configuration = Configuration {
                state: current_state,
                tape: &tape,
            };
            println!("{}", configuration);

            // If we are in the accepting state, immediately terminate
            // the machine.
            if self.accepting_states.contains(&current_state) {
                return Ok(true);
            }

            // Read the current symbol.
            let current_symbol = tape.read();

            // Apply the transition function based on the current state
            // and the current symbol.
            let input = (current_state, current_symbol);
            let maybe_output = self.transition_function.get(&input);
            let output = maybe_output.ok_or(RuntimeError {
                message: "something went wrong".to_string(),
            })?;

            current_state = output.0;
            let write_symbol = output.1;
            let direction = &output.2;

            // Write the symbol.
            tape.write(write_symbol);

            // Move the head.
            tape.move_head(&direction);
        }
    }
}

fn main() {
    // States
    let a = State { s: 'a' };
    let b = State { s: 'b' };
    let c = State { s: 'c' };
    let halt = State { s: 'h' };
    let mut states = HashSet::new();
    states.insert(a);
    states.insert(b);
    states.insert(c);
    states.insert(halt);

    // Tape alphabet
    let zero = Symbol { s: '0' };
    let one = Symbol { s: '1' };
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
    let machine = Machine {
        states,
        tape_alphabet,
        blank_symbol,
        input_alphabet,
        initial_state,
        accepting_states,
        transition_function,
    };

    // Run on the initially empty tape.
    let mut data = VecDeque::new();
    data.push_back(zero);
    let tape = Tape::new(data, blank_symbol);
    machine.run(tape);
}
