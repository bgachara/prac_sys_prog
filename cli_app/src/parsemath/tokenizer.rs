//tokenizer is the module that reads one or more arithmetic expressions from an expression and translates into a token.
//create a ds to store input expressions and represent output tokens.
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}
