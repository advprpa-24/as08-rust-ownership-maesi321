use crate::term::*;
use std::{arch::x86_64::_SIDD_LEAST_SIGNIFICANT, fmt};

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    // TODO: Implement pretty printing for lambda calculus terms.
    // format!("{:?}", term)

    match term {
        Term::Var(var) => format!("{}", var),
        Term::Abs(arg, body) => {
            let body_print = pretty_print(body);
            format!("λ{}.{}", arg, body_print)
        } 
        Term::App(left, right) => {
            let left_print = pretty_print(left);
            let right_print = pretty_print(right);
            format!("({} {})", left_print, right_print)
        }
    }
    
    
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}