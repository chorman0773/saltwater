#[macro_use]
mod macros;
mod arch;
mod analyze;
mod fold;
mod lex;
mod parse;

pub use parse::Parser;
use saltwater_core::ErrorHandler;

use std::rc::Rc;

#[derive(Clone, Debug, Default)]
struct RecursionGuard(Rc<()>);

impl RecursionGuard {
    // this is just a guesstimate, it should probably be configurable
    #[cfg(debug_assertions)]
    const MAX_DEPTH: usize = 1000;
    #[cfg(not(debug_assertions))]
    const MAX_DEPTH: usize = 10000;

    // make sure we don't crash on highly nested expressions
    // or rather, crash in a controlled way
    fn recursion_check(&self, error_handler: &mut ErrorHandler) -> RecursionGuard {
        let guard = self.clone();
        let depth = Rc::strong_count(&guard.0);
        if depth > Self::MAX_DEPTH {
            Self::recursion_check_failed(depth, error_handler);
        }
        guard
    }

    #[cold]
    #[inline(never)]
    fn recursion_check_failed(depth: usize, mut error_handler: &mut ErrorHandler) -> ! {
        eprintln!(
            "fatal: maximum recursion depth exceeded ({} > {})",
            depth,
            Self::MAX_DEPTH
        );
        if !error_handler.is_empty() {
            println!("pending errors:");
            for error in &mut error_handler {
                println!("- error: {}", error.data);
            }
            for warning in &mut error_handler.warnings {
                println!("- warning: {}", warning.data);
            }
        }
        std::process::exit(102);
    }
}
