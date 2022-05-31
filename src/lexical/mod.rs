use std::collections::{HashSet, LinkedList};
use std::iter::Map;

pub struct Lexical {
    r#type: LexicalType,
}

enum LexicalType {}

trait LexicalEntry {
    fn chars() -> HashSet<char>;
    fn function<O: Fn() -> Result<LinkedList<Lexical>, String>>(c: char) -> O;
}
