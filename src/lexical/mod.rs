mod identifier;

use crate::lexical::identifier::Identifier;
use crate::{LexResult, LexerError, LexicalOperations};
use std::collections::{HashSet, LinkedList};
use std::fmt::Debug;
use std::iter::Map;

pub trait LexicalDefinition {
    const NAME: &'static str;
    const LEX_START: &'static str;
    const LEX_ENDING: Option<&'static str>;
    const SUB_LEXICAL: Option<&'static [dyn LexicalDefinition + default]>;
    type LexicalOperation: LexicalToken;

    fn operate(
        collect: String,
        last: char,
        current: char,
        next: char,
    ) -> Result<Self::LexicalOperation, LexerError>;
}

trait LexicalToken {}

pub struct LexicalRoot {}

impl LexicalToken for LexicalRoot {}

impl LexicalDefinition for LexicalRoot {
    const NAME: &'static str = "Root";
    const LEX_START: &'static str = "";
    const LEX_ENDING: Option<&'static str> = None;
    const SUB_LEXICAL: Option<&'static [dyn LexicalDefinition + Default]> = Some(&[Identifier]);
    type LexicalOperation = Self;

    fn operate(
        collect: String,
        last: char,
        current: char,
        next: char,
    ) -> Result<Self::LexicalOperation, LexerError> {
        for lex in Self::SUB_LEXICAL.ok_or(LexerError::NoLexicalRoutine)? {}
        Err(LexerError::NoLexicalRoutine)
    }
}
