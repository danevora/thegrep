pub mod helpers;

// Starter code for PS06 - thegrep
use self::State::*;
/**
 * Author(s): Daniel Evora, Peter Morrow
 * Onyen(s): devora, peterjm
 *
 * UNC Honor Pledge: I pledge I have recieved no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

//imports for operator overloading
use std::ops::Add;

/**
 * ===== Public API =====
 */

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug, Clone)]
pub struct NFA {
    start: StateId,
    states: Vec<State>,
}

impl NFA {
    /**
     * Construct an NFA from a regular expression pattern.
     */
    pub fn from(regular_expression: &str) -> Result<NFA, String> {
        let mut nfa = NFA::new();

        let start = nfa.add_state(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add_state(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */

    pub fn accepts(&self, input: &str) -> bool {
        let curr_state = self.start; // sets the current state to the start
        let chars = input.chars(); // creating an iterator over the characters of the input
        self.recur(curr_state, chars) // calls recursive helper function where StateId and iterator state are kept track of
    }

    pub fn recur(&self, mut curr_state: StateId, mut chars: std::str::Chars) -> bool {
        match &self.states[curr_state] {
            // matches the current state to one of State's enums
            State::Start(Some(id)) => {
                // if the curr_state is the Start state, this is matched
                curr_state = *id; // curr_state is now set to whatever state self.start points to
                self.recur(curr_state, chars) // recusrive call to start testing input from state right after the start state
            }
            State::Match(expected_char, Some(id)) => match expected_char {
                // if the curr_state is a Match state, this is matched
                Char::Literal(c) => {
                    if let Some(letter) = chars.next() {
                        // checks to make sure there is something left in input and moves forward on iterator
                        if letter == *c {
                            // checks to see if the next letter in input is equal to the character matched by c
                            curr_state = *id; // curr_state changes to wherever curr_state points to
                            self.recur(curr_state, chars) // recursive call
                        } else {
                            // self.recur(self.start, chars)
                            false
                        }
                    } else {
                        false // false if the input ends and there was never a match
                    }
                }
                Char::Any => {
                    curr_state = *id; // curr_state is wherever curr_state currently points to
                    if let Some(c) = chars.next() {
                        // moves iterator to nect letter in input
                        self.recur(curr_state, chars)
                    } else {
                        false
                    } // recursive call
                }
            },
            State::Split(Some(leg_one), Some(leg_two)) => {
                let clone = chars.clone(); // clones iterator since chars is mutable and we need to test two (or more) possibilities for chars
                self.recur(*leg_one, chars) || self.recur(*leg_two, clone)
            }
            State::End => true, // if the State is the End state, we know that the input is accepted (base case here)
            _ => false,         // if there is any other state, that means return false
        }
    }

    /**
     * Gen function generates acceptable strings given a regular expression. 
     * recur_gen is a recursive helper method used in gen
     */

    pub fn gen(&self) -> String {
        let start = self.start;
        let mut input = String::new();
        self.recur_gen(start, input)
    }

    pub fn recur_gen(&self, mut curr_state: StateId, mut input: String) -> String {
        match &self.states[curr_state] {
            State::Start(Some(id)) => {
                curr_state = *id;
                self.recur_gen(curr_state, input)
            }
            State::Match(expected_char, Some(id)) => match expected_char {
                Char::Literal(c) => {
                    curr_state = *id;
                    input.push(*c);
                    self.recur_gen(curr_state, input)
                }
                Char::Any => {
                    curr_state = *id;
                    let mut rng = thread_rng();
                    let c: char = rng.sample(&Alphanumeric);
                    input.push(c);
                    self.recur_gen(curr_state, input)
                }
            },
            State::Split(Some(leg_one), Some(leg_two)) => {
                let choice: f64 = rand::thread_rng().gen();
                if choice < 0.5 {
                    curr_state = *leg_one;
                    self.recur_gen(curr_state, input)
                } else {
                    curr_state = *leg_two;
                    self.recur_gen(curr_state, input)
                }
            }
            State::End => input,
            _ => panic!("Unexpected state in NFA"),
        }
    }
}

#[cfg(test)]
mod public_api {
    use super::*;

    #[test]
    fn simple() {
        let input = NFA::from("a").unwrap();
        assert_eq!(input.accepts("a"), true);
        assert_eq!(input.accepts("b"), false);
    }

    #[test]
    fn catenation() {
        let input = NFA::from("abc").unwrap();
        assert_eq!(input.accepts("abc"), true);
        assert_eq!(input.accepts("abd"), false);
        assert_eq!(input.accepts("adc"), false);
        assert_eq!(input.accepts("dbc"), false);
        assert_eq!(input.accepts("cba"), false);
    }

    #[test]
    fn simple_alternation() {
        let input = NFA::from("a|b").unwrap();
        assert_eq!(input.accepts("a"), true);
        assert_eq!(input.accepts("b"), true);
    }

    #[test]
    fn alt_with_cat() {
        let input = NFA::from("ab|ac").unwrap();
        assert_eq!(input.accepts("ab"), true);
        assert_eq!(input.accepts("ac"), true);
        assert_eq!(input.accepts("bc"), false);
        assert_eq!(input.accepts("bb"), false);
        assert_eq!(input.accepts("cc"), false);
        assert_eq!(input.accepts("aa"), false);
        let input = NFA::from("a|bc").unwrap();
        assert_eq!(input.accepts("a"), true);
        assert_eq!(input.accepts("bc"), true);
        assert_eq!(input.accepts("ac"), true);
        assert_eq!(input.accepts("bb"), false);
    }

    #[test]
    fn multiple_alts() {
        let input = NFA::from("a|b|cd").unwrap();
        assert_eq!(input.accepts("a"), true);
        assert_eq!(input.accepts("b"), true);
        assert_eq!(input.accepts("cd"), true);
        assert_eq!(input.accepts("ad"), true);
        assert_eq!(input.accepts("bd"), true);
        assert_eq!(input.accepts("cb"), true);
        assert_eq!(input.accepts("ca"), true);
        assert_eq!(input.accepts("cc"), false);
    }

    #[test]
    fn input_with_any() {
        let input = NFA::from("a...b").unwrap();
        assert_eq!(input.accepts("ab"), false);
        assert_eq!(input.accepts("a   b"), true);
        assert_eq!(input.accepts("axyzb"), true);
        assert_eq!(input.accepts("xyzb"), false);
        assert_eq!(input.accepts("axyz"), false);
    }

    #[test]
    fn simple_closure() {
        let input = NFA::from("a*").unwrap();
        assert_eq!(input.accepts(""), true);
        assert_eq!(input.accepts("ab"), true);
        assert_eq!(input.accepts("aaaaaaa"), true);
    }

    #[test]
    fn more_closure() {
        let input = NFA::from("ab*|c*a").unwrap();
        assert_eq!(input.accepts("a"), true);
        assert_eq!(input.accepts("abbbbbbb"), true);
        assert_eq!(input.accepts("ccccccca"), true);
        assert_eq!(input.accepts("abbbbbbcccc"), true);
        assert_eq!(input.accepts("ccccccabbbb"), true);
        assert_eq!(input.accepts("bbbbccccbbb"), false);
        assert_eq!(input.accepts("abbbbccccca"), true);
        assert_eq!(input.accepts("aa"), true);
    }
}


impl Add for NFA {
    type Output = NFA;

    fn add(self, rhs: NFA) -> NFA {
        // clone self and rhs
        // take self's state's length and add that num to each id in the rhs
        let mut lhs = self.clone();
        let mut rhs_clone = rhs.clone();
        let offset = lhs.states.len();
        lhs.states.pop();
        for mut s in &rhs_clone.states {
            match s {
                State::Start(Some(id)) => {
                    s = &State::Start(Some(id + offset));
                },
                State::Match(c, Some(id)) => {
                    s = &State::Match(c.clone(), Some(id + offset));
                },
                State::Split(Some(id_one), Some(id_two)) => {
                    s = &State::Split(Some(id_one + offset), Some(id_two + offset));
                },
                State::End => break,
                _ => panic!("Unexpected state in NFA"),
            }
        }
        lhs.states.append(&mut rhs_clone.states);
        return lhs;
    }
}



/**
 * ===== Internal API =====
 */
type StateId = usize;

/**
 * States are the elements of our NFA Graph
 * - Start is starting state
 * - Match is a state with a single matching transition out
 * - Split is a state with two epsilon transitions out
 * - End is the final accepting state
 */
#[derive(Debug, Clone)]
enum State {
    Start(Option<StateId>),
    Match(Char, Option<StateId>),
    Split(Option<StateId>, Option<StateId>),
    End,
}

/**
 * Chars are the matching label of a non-epsilon edge in the
 * transition diagram representation of the NFA.
 */
#[derive(Debug, Clone)]
enum Char {
    Literal(char),
    Any,
}

/**
 * Internal representation of a fragment of an NFA being constructed
 * that keeps track of the start ID of the fragment as well as all of
 * its unjoined end states.
 */
#[derive(Debug)]
struct Fragment {
    start: StateId,
    ends: Vec<StateId>,
}

/**
 * Private methods of the NFA structure.
 */
impl NFA {
    /**
     * Constructor establishes an empty states Vec.
     */
    fn new() -> NFA {
        NFA {
            states: vec![],
            start: 0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back.
     */
    fn add_state(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * Given an AST node, this method returns a Fragment of the NFA
     * representing it and its children.
     */
    fn gen_fragment(&mut self, ast: &AST) -> Fragment {
        match ast {
            AST::AnyChar => {
                let state = self.add_state(Match(Char::Any, None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Char(c) => {
                let state = self.add_state(Match(Char::Literal(*c), None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Catenation(lhs, rhs) => {
                let frag_one = self.gen_fragment(&lhs);
                let frag_two = self.gen_fragment(&rhs);
                self.join_fragment(&frag_one, frag_two.start);
                Fragment {
                    start: frag_one.start,
                    ends: frag_two.ends,
                }
            }
            AST::Alternation(lhs, rhs) => {
                let mut frag_one = self.gen_fragment(&lhs);
                let mut frag_two = self.gen_fragment(&rhs);
                let state = self.add_state(Split(Some(frag_one.start), Some(frag_two.start)));
                frag_one.ends.append(&mut frag_two.ends);
                Fragment {
                    start: state,
                    ends: frag_one.ends,
                }
            }
            AST::Closure(expr) => {
                let frag = self.gen_fragment(&expr);
                let state = self.add_state(Split(Some(frag.start), None));
                self.join_fragment(&frag, state);
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            } 
            AST::OneOrMore(expr) => {
                let frag = self.gen_fragment(&expr);
                let state = self.add_state(Split(Some(frag.start), None));
                self.join_fragment(&frag, state);
                Fragment {
                    start: frag.start,
                    ends: vec![state],
                }
            }
            
        }
    }

    /**
     * Join all the loose ends of a fragment to another StateId.
     */
    fn join_fragment(&mut self, lhs: &Fragment, to: StateId) {
        for end in &lhs.ends {
            self.join(*end, to);
        }
    }

    /**
     * Join a loose end of one state to another by IDs.
     * Note in the Split case, only the 2nd ID (rhs) is being bound.
     * It is assumed when building an NFA with these constructs
     * that the lhs of an Split state will always be known and bound.
     */
    fn join(&mut self, from: StateId, to: StateId) {
        match self.states[from] {
            Start(ref mut next) => *next = Some(to),
            Match(_, ref mut next) => *next = Some(to),
            Split(_, ref mut next) => *next = Some(to),
            End => {}
        }
    }
}
