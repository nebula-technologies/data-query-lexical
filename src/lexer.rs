use crate::lexer_constants::*;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq)]
pub enum LexerError {
    EndOfQuery {
        expected: String,
        char_pointer: usize,
        lex: String,
    },
    FailedToParseInt(ParseIntError),
    UnexpectedCharacter {
        expected: String,
        found: String,
        char_pointer: usize,
        lex: String,
    },
}

impl From<ParseIntError> for LexerError {
    fn from(e: ParseIntError) -> Self {
        Self::FailedToParseInt(e)
    }
}

pub type LexResult<T> = Result<T, LexerError>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GenericObjectIndex {
    Wildcard,
    Slice(LinkedList<Slicer>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Slicer {
    Index(usize),
    Slice(usize, usize),
    Ident(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LexOperator {
    Identifier(String),
    Pipe(LinkedList<LexOperator>),
    Generic(GenericObjectIndex),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LexicalOperations(LinkedList<LexOperator>);

impl From<LinkedList<LexOperator>> for LexicalOperations {
    fn from(v: LinkedList<LexOperator>) -> Self {
        Self(v)
    }
}

impl Deref for LexicalOperations {
    type Target = LinkedList<LexOperator>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LexicalOperations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryInto<LexicalOperations> for &str {
    type Error = String;

    fn try_into(self) -> Result<LexicalOperations, String> {
        compile(self).map_err(|e| format!("{:?}", e))
    }
}

pub fn compile(s: &str) -> LexResult<LexicalOperations> {
    let mut lexer_vec = s.chars().into_iter().collect::<Vec<char>>();
    lexer_vec.reverse();
    generic_compiler(
        &mut lexer_vec,
        &mut Default::default(),
        Default::default(),
        false,
        Default::default(),
    )
    .map(LexicalOperations::from)
}

pub fn generic_compiler(
    lexer_vec: &mut Vec<char>,
    mut operator: &mut LinkedList<LexOperator>,
    mut collect: String,
    mut escape: bool,
    mut char_pointer: usize,
) -> LexResult<LinkedList<LexOperator>> {
    let char = lexer_vec.pop();
    if let Some(c) = char {
        char_pointer = char_pointer + 1;
        if !escape {
            match c {
                LEX_ESCAPE => {
                    escape = true;
                }
                LEX_IDENTIFIER => {
                    if !collect.is_empty() {
                        operator.push_back(LexOperator::Identifier(collect));
                    }
                    collect = Default::default();
                }
                LEX_GENERIC_START => {
                    if !collect.is_empty() {
                        operator.push_back(LexOperator::Identifier(collect));
                        collect = Default::default();
                    }
                    let v = generic_object_index(
                        lexer_vec,
                        Default::default(),
                        LinkedList::new(),
                        None,
                        false,
                        char_pointer,
                    )?;
                    operator.push_back(LexOperator::Generic(v));
                }
                _ => {
                    if c != LEX_ROUGE_WIDESPACE {
                        collect.push(c)
                    }
                }
            }
        } else {
            collect.push(c);
            escape = false;
        }
        generic_compiler(lexer_vec, operator, collect, escape, char_pointer)
    } else {
        Ok(operator.clone())
    }
}

fn generic_object_index(
    lexer_vec: &mut Vec<char>,
    mut collect: String,
    mut slicer: LinkedList<Slicer>,
    mut tmp_slice: Option<usize>,
    mut escape: bool,
    mut char_pointer: usize,
) -> LexResult<GenericObjectIndex> {
    let char = lexer_vec.pop();
    if let Some(c) = char {
        char_pointer += 1;
        if !escape {
            match c {
                LEX_ESCAPE => {
                    generic_object_index(lexer_vec, collect, slicer, tmp_slice, true, char_pointer)
                }
                LEX_GENERIC_END => {
                    if collect.is_empty() && slicer.is_empty() {
                        Ok(GenericObjectIndex::Wildcard)
                    } else if !collect.is_empty() {
                        if let Some(from) = tmp_slice {
                            let to = collect.parse::<usize>().map_err(LexerError::from)?;
                            slicer.push_back(Slicer::Slice(from, to));
                            tmp_slice = None;
                        } else if let Ok(u) = collect.parse::<usize>() {
                            slicer.push_back(Slicer::Index(u));
                        } else {
                            slicer.push_back(Slicer::Ident(collect.clone()));
                        }
                        Ok(GenericObjectIndex::Slice(slicer))
                    } else {
                        Ok(GenericObjectIndex::Slice(slicer))
                    }
                }
                LEX_GENERIC_SEPARATOR => {
                    if collect.is_empty() && slicer.is_empty() {
                        Err(LexerError::UnexpectedCharacter {
                            expected: "Integer/String".to_string(),
                            found: LEX_GENERIC_SEPARATOR.to_string(),
                            char_pointer,
                            lex: format!("{:?}", lexer_vec),
                        })
                    } else {
                        if let Some(from) = tmp_slice {
                            let to = collect.parse::<usize>().map_err(LexerError::from)?;
                            slicer.push_back(Slicer::Slice(from, to));
                            tmp_slice = None;
                        } else if let Ok(u) = collect.parse::<usize>() {
                            slicer.push_back(Slicer::Index(u));
                        } else {
                            slicer.push_back(Slicer::Ident(collect.clone()));
                        }
                        collect = Default::default();
                        generic_object_index(
                            lexer_vec,
                            collect,
                            slicer,
                            tmp_slice,
                            false,
                            char_pointer,
                        )
                    }
                }
                LEX_GENERIC_SLICE => {
                    if collect.is_empty() && slicer.is_empty() {
                        return Err(LexerError::UnexpectedCharacter {
                            expected: "Integer/String".to_string(),
                            found: LEX_GENERIC_SEPARATOR.to_string(),
                            char_pointer,
                            lex: format!("{:?}", lexer_vec),
                        });
                    } else if let Ok(u) = collect.parse::<usize>() {
                        tmp_slice = Some(u);
                    } else {
                        return Err(LexerError::UnexpectedCharacter {
                            expected: "Integer".to_string(),
                            found: "String".to_string(),
                            char_pointer,
                            lex: format!("{:?}", lexer_vec),
                        });
                    }
                    collect = Default::default();
                    generic_object_index(lexer_vec, collect, slicer, tmp_slice, false, char_pointer)
                }
                LEX_ROUGE_WIDESPACE => {
                    generic_object_index(lexer_vec, collect, slicer, tmp_slice, false, char_pointer)
                }
                _ => {
                    collect.push(c);
                    generic_object_index(lexer_vec, collect, slicer, tmp_slice, false, char_pointer)
                }
            }
        } else {
            collect.push(c);
            generic_object_index(lexer_vec, collect, slicer, tmp_slice, false, char_pointer)
        }
    } else {
        Err(LexerError::EndOfQuery {
            expected: String::from(LEX_GENERIC_END),
            char_pointer,
            lex: format!("{:?}", lexer_vec),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::LexOperator::*;
    use crate::lexer::Slicer::*;
    use crate::lexer::{
        compile, generic_compiler, generic_object_index, GenericObjectIndex, LexOperator,
        LexResult, Slicer,
    };
    use crate::LexicalOperations;
    use std::collections::LinkedList;

    fn lex_vec(s: &str) -> Vec<char> {
        s.chars().into_iter().collect::<Vec<char>>()
    }

    #[test]
    pub fn test_slicer() {
        let mut lex_vec = lex_vec("1,2,4-6,hello]");
        lex_vec.reverse();
        let slicer = generic_object_index(
            &mut lex_vec,
            "".to_string(),
            LinkedList::new(),
            None,
            false,
            0usize,
        );
        let true_generic_object = GenericObjectIndex::Slice(LinkedList::from([
            Slicer::Index(1),
            Slicer::Index(2),
            Slicer::Slice(4, 6),
            Ident("hello".to_string()),
        ]));

        assert_eq!(true_generic_object, slicer.unwrap())
    }

    #[test]
    pub fn test_generic_compiler() {
        let mut lex_vec = lex_vec(".metadata[1,2,4-6,hello]");
        lex_vec.reverse();
        let mut operator = LinkedList::new();
        let compiled_lex = generic_compiler(
            &mut lex_vec,
            &mut operator,
            Default::default(),
            false,
            Default::default(),
        );
        let true_result: LexResult<LinkedList<LexOperator>> = Ok(LinkedList::from([
            Identifier("metadata".to_string()),
            Generic(GenericObjectIndex::Slice(LinkedList::from([
                Index(1),
                Index(2),
                Slice(4, 6),
                Ident("hello".to_string()),
            ]))),
        ]));
        assert_eq!(true_result, compiled_lex);
    }

    #[test]
    pub fn test_compiler() {
        let compiled_lex = compile(".metadata[1,2,4-6,hello]");
        let true_result: LexResult<LexicalOperations> = Ok(LinkedList::from([
            Identifier("metadata".to_string()),
            Generic(GenericObjectIndex::Slice(LinkedList::from([
                Index(1),
                Index(2),
                Slice(4, 6),
                Ident("hello".to_string()),
            ]))),
        ])
        .into());
        assert_eq!(true_result, compiled_lex);
    }

    #[test]
    pub fn test_lex_escape() {
        let compiled_lex = compile(".metadata[1,2\\,,4-6,hello]");
        let true_result: LexResult<LexicalOperations> = Ok(LinkedList::from([
            Identifier("metadata".to_string()),
            Generic(GenericObjectIndex::Slice(LinkedList::from([
                Index(1),
                Ident("2,".to_string()),
                Slice(4, 6),
                Ident("hello".to_string()),
            ]))),
        ])
        .into());
        assert_eq!(true_result, compiled_lex);
    }

    #[test]
    pub fn test_lex_escape_identifier() {
        let compiled_lex = compile(".meta\\.data[1,2\\,,4-6,hello]");
        let true_result: LexResult<LexicalOperations> = Ok(LinkedList::from([
            Identifier("meta.data".to_string()),
            Generic(GenericObjectIndex::Slice(LinkedList::from([
                Index(1),
                Ident("2,".to_string()),
                Slice(4, 6),
                Ident("hello".to_string()),
            ]))),
        ])
        .into());
        assert_eq!(true_result, compiled_lex);
    }
}

/*


{
  "default": "Personal",
  "annotation-field": "annotations",
  "workspaces": {
    "Personal": {
      "HelloWorld": {
        "annotations": {
          "my-app.io/group": "HelloWorld"
        }
      },
      "NoWorld": {}
    }
  }
}



 */
