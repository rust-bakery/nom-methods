//! Method macro combinators
//!
//! These macros make parsers as methods of structs
//! and that can take methods of structs to call
//! as parsers.
//!
//! There is a trick to make them easier to assemble,
//! combinators are defined like this:
//!
//! ```ignore
//! macro_rules! tag (
//!   ($i:expr, $inp: expr) => (
//!     {
//!       ...
//!     }
//!   );
//! );
//! ```
//!
//! But when used as methods in other combinators, are used
//! like this:
//!
//! ```ignore
//! method!(my_function<Parser<'a> >, self, tag!("abcd"));
//! ```
//!
//! Internally, other combinators will rewrite
//! that call to pass the input as second argument:
//!
//! ```ignore
//! macro_rules! method (
//!   ($name:ident<$a:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
//!     fn $name( $self_: $a, i: &[u8] ) -> IResult<&[u8], &[u8]> {
//!       $submac!(i, $($args)*)
//!     }
//!   );
//! );
//! ```
//!
//! The `method!` macro is similar to the `named!` macro in the macros module.
//! While `named!` will create a parser function, `method!` will create a parser
//! method on the struct it is defined in.
//!
//! Compared to the `named!` macro there are a few differences in how they are
//! invoked. A `method!` invocation always has to have the type of `self`
//! declared and it can't be a reference due to Rust's borrow lifetime
//! restrictions:
//!
//! ```ignore
//! //                  -`self`'s type-
//! method!(method_name<  Parser<'a> >, ...);
//! ```
//! `self`'s type always comes first.
//! The next difference is you have to input the self struct. Due to Rust's
//! macro hygiene the macro can't declare it on it's own.
//!
//! ```ignore
//! //                                                 -self-
//! method!(method_name<Parser<'a>, &'a str, &'a str>, self, ...);
//! ```
//! When making a parsing struct with parsing methods, due to the static borrow
//! checker,calling any parsing methods on self (or any other parsing struct)
//! will cause self to be moved for the rest of the method.To get around this
//! restriction all self is moved into the called method and then the called
//! method will return self to the caller.
//!
//! To call a method on self you need to use the `call_m!` macro. For example:
//!
//! ```ignore
//! struct<'a> Parser<'a> {
//!   parsed: &'a str,
//! }
//! impl<'a> Parser<'a> {
//!   // Constructor omitted for brevity
//!   method!(take4<Parser<'a>, &'a str, &'a str>, self, take!(4));
//!   method!(caller<Parser<'a>, &'a str, &'a str>, self, call_m!(self.take4));
//! }
//! ```
//! More complicated combinations still mostly look the same as their `named!`
//! counterparts:
//!
//! ```ignore
//!    method!(pub simple_chain<&mut Parser<'a>, &'a str, &'a str>, self,
//!      do_parse!(
//!             call_m!(self.tag_abc)                                        >>
//!             call_m!(self.tag_def)                                        >>
//!             call_m!(self.tag_ghi)                                        >>
//!       last: map!(call_m!(self.simple_peek), |parsed| sb.parsed = parsed) >>
//!       (last)
//!      )
//!    );
//! ```
//! The three additions to method definitions to remember are:
//! 1. Specify `self`'s type
//! 2. Pass `self` to the macro
//! 4. Call parser methods using the `call_m!` macro.

#[macro_use]
extern crate nom;

pub mod lib {
  pub use nom::{IResult, error::Error};
}

pub use self::methods::*;

#[macro_use]
mod methods;
