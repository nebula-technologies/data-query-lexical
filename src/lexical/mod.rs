mod identifier;

// use crate::lexical::identifier::Identifier;
use crate::lexical::identifier::{Identifier, IDENTIFIER};
use crate::{LexResult, LexerError};
use std::collections::{HashSet, LinkedList};
use std::fmt::Debug;
use std::iter::Map;
use std::ops::{Deref, DerefMut};

pub trait LexicalDefinition {
    fn name(self: &Self) -> &str {
        ""
    }

    fn lex_start(self: &Self) -> &str {
        ""
    }

    fn lex_end(self: &Self) -> Option<&str> {
        None
    }

    fn sub_lexical(self: &Self) -> Option<Vec<&'static dyn LexicalDefinition>> {
        None
    }

    fn operate(
        self: &Self,
        pre_res: &mut Option<String>,
        cur_res: &mut Option<String>,
        lex_char: &mut TmpChar,
    ) -> Result<&dyn LexicalDefinition, LexerError>;
}

pub struct LexicalRoot {}

impl Default for LexicalRoot {
    fn default() -> Self {
        Self {}
    }
}

impl LexicalDefinition for LexicalRoot {
    fn name(&self) -> &str {
        "Root"
    }

    fn lex_start(&self) -> &str {
        ""
    }

    fn lex_end(&self) -> Option<&str> {
        None
    }

    fn sub_lexical(&self) -> Option<Vec<&'static dyn LexicalDefinition>> {
        //Some(vec![&IDENTIFIER])
        None
    }

    fn operate(
        &self,
        mut pre_res: &mut Option<String>,
        mut cur_res: &mut Option<String>,
        lex_char: &mut TmpChar,
    ) -> Result<&dyn LexicalDefinition, LexerError> {
        while lex_char.get_current().is_some() {
            for lex in self.sub_lexical().ok_or(LexerError::NoLexicalRoutine)? {
                if lex_char.compare_current(self.lex_start()) {
                    // let last = current;
                    // let current = next;
                    // let mut next = lexical_str.pop();
                    lex.operate(pre_res, cur_res, lex_char);
                }
            }
            // let last = current;
            // let current = next;
            // let mut next = lexical_str.pop();
            cur_res = &mut match (cur_res, lex_char.get_previous()) {
                (Some(t), Some(c)) => Some(t.clone() + c.to_string().as_str()),
                (Some(t), _) => Some(t.clone()),
                (_, Some(c)) => Some(c.to_string()),
                _ => None,
            };
        }
        Err(LexerError::NoLexicalRoutine)
    }
}

pub struct TmpChar {
    lexical: Vec<char>,
    pointer: usize,
    pre: Option<char>,
    cur: Option<char>,
    nxt: Option<char>,
}

impl TmpChar {
    fn next(&mut self) {
        if self.pointer + 1 < self.lexical.len() {
            self.pointer += 1;
        }
    }

    fn last(&mut self) {
        if self.pointer - 1 >= 0 {
            self.pointer -= 1;
        }
    }

    fn get_next(&self) -> Option<&char> {
        self.lexical.get(self.pointer + 1)
    }

    fn get_current(&self) -> Option<&char> {
        self.lexical.get(self.pointer)
    }

    fn get_previous(&self) -> Option<&char> {
        self.lexical.get(self.pointer - 1)
    }

    fn compare_current(&self, c: &str) -> bool {
        match self.get_current() {
            Some(t) => t.to_string().as_str() == c,
            None => false,
        }
    }

    fn compare_next(&self, c: &str) -> bool {
        match self.get_next() {
            Some(t) => t.to_string().as_str() == c,
            None => false,
        }
    }

    fn compare_previous(&self, c: &str) -> bool {
        match self.get_previous() {
            Some(t) => c.to_string().as_str() == c,
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexical::{LexicalDefinition, LexicalRoot};

    // fn test_root_lexical() {
    //     let lr = LexicalRoot {};
    //     let mut lexical_str = "hello".chars().collect::<Vec<char>>();
    //     lr.operate(
    //         &mut lexical_str,
    //         &mut None,
    //         &mut None,
    //         &mut None,
    //         &mut None,
    //         &mut None,
    //     );
    // }
}
