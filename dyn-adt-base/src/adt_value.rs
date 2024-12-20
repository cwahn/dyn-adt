// src/adt_value.rs

use crate::{adt_id::AdtId, adt_ir::AdtIr};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace0, multispace1},
    combinator::{all_consuming, map, opt, recognize},
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdtValue {
    pub adt_id: AdtId,
    pub ir: AdtIr,
}

// impl AdtValue {
//     // /// Matches the `AdtValue` against a pattern string.
//     // /// Returns `true` if the pattern matches, `false` otherwise.
//     // pub fn pattern_match(&self, pattern: &str) -> Result<bool, String> {
//     //     // Parse the pattern string into a `Pattern` enum
//     //     let pattern = parse_pattern_full(pattern)?;

//     //     // Perform the matching logic
//     //     Ok(self.ir.matches(&pattern))
//     // }
// }

// todo implement pattern matching

// /// Haskell-like pattern matching
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum Pattern<'a> {
//     Wildcard,
//     Variable(&'a str),
//     Tuple(Vec<Pattern<'a>>),
//     Constructor(&'a str, Vec<Pattern<'a>>),
// }

// /// Parses an input string into a `Pattern` enum.
// pub fn parse_pattern_full(input: &str) -> Result<Pattern, String> {
//     match all_consuming(parse_pattern)(input) {
//         Ok((_, pattern)) => Ok(pattern),
//         Err(e) => Err(format!("Parse error: {}", e)),
//     }
// }

// /// The main pattern parser.
// fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
//     alt((
//         parse_wildcard,
//         parse_tuple,
//         parse_constructor,
//         parse_variable,
//     ))(input)
// }

// /// Parses a wildcard pattern "_".
// fn parse_wildcard(input: &str) -> IResult<&str, Pattern> {
//     map(tag("_"), |_| Pattern::Wildcard)(input)
// }

// /// Parses a variable pattern (starts with lowercase).
// fn parse_variable(input: &str) -> IResult<&str, Pattern> {
//     map(
//         recognize(tuple((
//             take_while1(|c: char| c.is_ascii_lowercase()),
//             take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
//         ))),
//         |s: &str| Pattern::Variable(s),
//     )(input)
// }

// /// Parses a constructor pattern (starts with uppercase).
// fn parse_constructor(input: &str) -> IResult<&str, Pattern> {
//     let (input, name) = recognize(tuple((
//         take_while1(|c: char| c.is_ascii_uppercase()),
//         take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
//     )))(input)?;

//     // Parse constructor arguments: either parenthesized list or brace-enclosed list
//     let (input, args) = opt(preceded(
//         multispace0,
//         alt((
//             delimited(
//                 char('('),
//                 separated_list0(
//                     preceded(multispace0, char(',')),
//                     preceded(multispace0, parse_pattern),
//                 ),
//                 char(')'),
//             ),
//             delimited(
//                 char('{'),
//                 separated_list0(
//                     preceded(multispace0, char(',')),
//                     preceded(multispace0, parse_pattern),
//                 ),
//                 char('}'),
//             ),
//         )),
//     ))(input)?;

//     let args = match args {
//         Some(a) => a,
//         None => Vec::new(),
//     };

//     Ok((input, Pattern::Constructor(name, args)))
// }

// /// Parses a tuple pattern "(x, y, ...)".
// fn parse_tuple(input: &str) -> IResult<&str, Pattern> {
//     let (input, patterns) = delimited(
//         preceded(multispace0, char('(')),
//         separated_list0(
//             preceded(multispace0, char(',')),
//             preceded(multispace0, parse_pattern),
//         ),
//         preceded(multispace0, char(')')),
//     )(input)?;

//     // Ensure that there is at least one element in the tuple
//     if patterns.is_empty() {
//         return Err(nom::Err::Error(nom::error::Error::new(
//             input,
//             nom::error::ErrorKind::Many1,
//         )));
//     }

//     Ok((input, Pattern::Tuple(patterns)))
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_wildcard() {
//         let input = "_";
//         let expected = Pattern::Wildcard;
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_variable() {
//         let input = "x";
//         let expected = Pattern::Variable("x");
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_variable_with_underscore() {
//         let input = "var_1";
//         let expected = Pattern::Variable("var_1");
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_tuple() {
//         let input = "(x, y)";
//         let expected = Pattern::Tuple(vec![Pattern::Variable("x"), Pattern::Variable("y")]);
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_tuple_with_spaces() {
//         let input = "( x , y )";
//         let expected = Pattern::Tuple(vec![Pattern::Variable("x"), Pattern::Variable("y")]);
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_constructor_no_args() {
//         let input = "Nothing";
//         let expected = Pattern::Constructor("Nothing", vec![]);
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_constructor_with_args_parentheses() {
//         let input = "Just(x)";
//         let expected = Pattern::Constructor("Just", vec![Pattern::Variable("x")]);
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_constructor_with_args_braces() {
//         let input = "Point { x, y }";
//         let expected = Pattern::Constructor(
//             "Point",
//             vec![Pattern::Variable("x"), Pattern::Variable("y")],
//         );
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_constructor_with_args_multiple_parentheses() {
//         let input = "Cons(x, xs)";
//         let expected = Pattern::Constructor(
//             "Cons",
//             vec![Pattern::Variable("x"), Pattern::Variable("xs")],
//         );
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_constructor_with_args_multiple_braces() {
//         let input = "Cons { x, xs }";
//         let expected = Pattern::Constructor(
//             "Cons",
//             vec![Pattern::Variable("x"), Pattern::Variable("xs")],
//         );
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_constructor_with_tuple() {
//         let input = "Pair(x, y)";
//         let expected =
//             Pattern::Constructor("Pair", vec![Pattern::Variable("x"), Pattern::Variable("y")]);
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_complex_pattern_constructor_with_args() {
//         let input = "Either(Left(x), y)";
//         let expected = Pattern::Constructor(
//             "Either",
//             vec![
//                 Pattern::Constructor("Left", vec![Pattern::Variable("x")]),
//                 Pattern::Variable("y"),
//             ],
//         );
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_complex_pattern_constructor_with_args_braces() {
//         let input = "Either { Left(x), y }";
//         let expected = Pattern::Constructor(
//             "Either",
//             vec![
//                 Pattern::Constructor("Left", vec![Pattern::Variable("x")]),
//                 Pattern::Variable("y"),
//             ],
//         );
//         let result = parse_pattern_full(input).unwrap();
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_invalid_pattern_trailing_comma() {
//         let input = "Just(x, )"; // Trailing comma
//         let result = parse_pattern_full(input);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_invalid_pattern_unmatched_parenthesis() {
//         let input = "Pair(x, y"; // Missing closing parenthesis
//         let result = parse_pattern_full(input);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_invalid_pattern_empty_tuple() {
//         let input = "()"; // Empty tuple
//         let result = parse_pattern_full(input);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_invalid_pattern_extra_tokens() {
//         let input = "Just(x, y) z"; // Extra argument
//         let result = parse_pattern_full(input);
//         assert!(result.is_err());
//     }
// }
