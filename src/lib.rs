extern crate proc_macro;

mod lexer;
pub(crate) mod lexer_constants;

pub use lexer::*;
use std::collections::LinkedList;

mod lexical;

pub trait MacroFormat {
    fn macro_fmt(&self) -> String;
}

impl MacroFormat for lexer::Slicer {
    fn macro_fmt(&self) -> String {
        match self {
            Slicer::Index(i) => format!("::data_query_lexical::Slicer::Index({})", i),
            Slicer::Slice(f, t) => format!("::data_query_lexical::Slicer::Slice({},{})", f, t),
            Slicer::Ident(i) => {
                format!("::data_query_lexical::Slicer::Ident(\"{}\".into())", i)
            }
        }
    }
}

impl MacroFormat for LinkedList<lexer::LexOperator> {
    fn macro_fmt(&self) -> String {
        format!(
            "::std::collections::LinkedList::from([{}])",
            self.iter()
                .map(|t| t.macro_fmt())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl MacroFormat for lexer::LexOperator {
    fn macro_fmt(&self) -> String {
        match self {
            LexOperator::Identifier(i) => {
                format!(
                    "::data_query_lexical::LexOperator::Identifier(\"{}\".into())",
                    i
                )
            }
            LexOperator::Pipe(p) => p.macro_fmt(),
            LexOperator::Generic(g) => format!(
                "::data_query_lexical::LexOperator::Generic({})",
                g.macro_fmt()
            ),
        }
    }
}

impl MacroFormat for lexer::GenericObjectIndex {
    fn macro_fmt(&self) -> String {
        match self {
            GenericObjectIndex::Wildcard => {
                format!("::data_query_lexical::GenericObjectIndex::Wildcard")
            }
            GenericObjectIndex::Slice(s) => format!(
                "::data_query_lexical::GenericObjectIndex::Slice(::std::collections::LinkedList::from([{}]))",
                s.iter()
                    .map(|s| s.macro_fmt())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}
