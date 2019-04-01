use std::iter::Peekable;
use std::str::Chars;

pub mod helpers;

// Starter code for PS06 - thegrep
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
use self::State::*;

/**
 * ===== Public API =====
 */

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug)]
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

        let start = nfa.add(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        let curr_state = self.start;
        let mut chars = input.chars();
        self.recur(curr_state, chars)
    } 

    pub fn recur(&self, mut curr_state: StateId, mut chars: std::str::Chars) -> bool {
            match &self.states[curr_state] {
                State::Start(Some(id)) => {
                    curr_state = *id;
                    self.recur(curr_state, chars)
                },
                State::Match(expected_char, Some(id)) => {
                    match expected_char {
                        Char::Literal(c) => {
                            if(chars.next() == Some(*c)) {
                                curr_state = *id;
                                self.recur(curr_state, chars)
                            } else {
                                false
                            }
                        },
                        _ => {
                            curr_state = *id;
                            self.recur(curr_state, chars)
                        },
                    }
                },
                State::Split(Some(leg_one), Some(leg_two)) => {
                    let clone = chars.clone();
                    if(self.recur(*leg_one, chars)) {
                        true
                    } else if (self.recur(*leg_two, clone)) {
                        true
                    } else {
                        false
                    }    
                },
                State::End => true,
                _ => false,
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
#[derive(Debug)]
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
#[derive(Debug)]
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
            start:  0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back.
     */
    fn add(&mut self, state: State) -> StateId {
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
                let state = self.add(Match(Char::Any, None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            },
            AST::Char(c) => {
                let state = self.add(Match(Char::Literal(*c), None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            },
            AST::Catenation(lhs, rhs) => {
                let frag_one = self.gen_fragment(&lhs);
                let frag_two = self.gen_fragment(&rhs);
                let frag = self.join_fragment(&frag_one, frag_two.start);
                Fragment {
                    start: frag_one.start,
                    ends: frag_two.ends,
                }
            },
            AST::Alternation(lhs, rhs) => {
                let mut frag_one = self.gen_fragment(&lhs);
                let mut frag_two = self.gen_fragment(&rhs);
                let state = self.add(Split(Some(frag_one.start), Some(frag_two.start)));
                frag_one.ends.append(&mut frag_two.ends);
                Fragment {
                 start: state,
                 ends: frag_one.ends,
                }
            },
            AST::Closure(expr) => {
                let mut frag = self.gen_fragment(&expr);
                let mut state = self.add(Split(Some(frag.start), None));
                self.join_fragment(&frag, state);
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            },
            node => panic!("Unimplemented branch of gen_fragment: {:?}", node)
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
