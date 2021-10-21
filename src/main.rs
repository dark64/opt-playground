#![feature(test)]
extern crate test;

use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashSet;

#[derive(Debug)]
struct Prog<T> {
    pub arguments: Vec<T>,
    pub statements: Vec<T>
}

// Our current folder trait
trait Folder<T>: Sized {
    // essentially this should be removed?
    fn fold_program(&mut self, p: Prog<T>) -> Prog<T> {
        Prog {
            arguments: p.arguments
                .into_iter()
                .map(|a| self.fold_argument(a))
                .collect(),
            statements: p.statements
                .into_iter()
                .filter_map(|s| self.fold_statement(s))
                .collect()
        }
    }
    fn fold_argument(&mut self, a: T) -> T {
        a
    }
    fn fold_statement(&mut self, s: T) -> Option<T> {
        Some(s)
    }
}


// A toy optimiser which removes duplicates
#[derive(Default)]
struct Deduplicator<T> {
    seen: HashSet<T>,
}

impl<T: Clone + Eq + Hash> Folder<T> for Deduplicator<T> {
    fn fold_statement(&mut self, s: T) -> Option<T> {
        if self.seen.get(&s).is_some() {
            None
        } else {
            self.seen.insert(s.clone());
            Some(s)
        }
    }
}

// A dummy optimiser just to have something to chain
#[derive(Default)]
struct DummyOptimizer;

impl<T> Folder<T> for DummyOptimizer {
    fn fold_argument(&mut self, a: T) -> T {
        a
    }
    fn fold_statement(&mut self, s: T) -> Option<T> {
        for _ in 0..100 {
            // do nothing
        }
        Some(s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn run_iter(b: &mut Bencher) {
        b.iter(|| {
            let prog = Prog {
                arguments: vec![1; 100],
                statements: vec![1; 100000]
            };

            let mut deduplicator = Deduplicator::default();
            let mut dummy = DummyOptimizer::default();

            let _ = Prog {
                arguments: prog
                    .arguments
                    .into_iter()
                    .map(|a| deduplicator.fold_argument(a))
                    .map(|a| dummy.fold_argument(a))
                    .collect(),
                statements: prog
                    .statements
                    .into_iter()
                    .filter_map(|s| deduplicator.fold_statement(s))
                    .filter_map(|s| dummy.fold_statement(s))
                    .collect()
            };
        });
    }

    #[bench]
    fn run_seq(b: &mut Bencher) {
        b.iter(|| {
            let prog = Prog {
                arguments: vec![1; 100],
                statements: vec![1; 100000]
            };

            let mut deduplicator = Deduplicator::default();
            let mut dummy = DummyOptimizer::default();

            let p = deduplicator.fold_program(prog);
            let _ = dummy.fold_program(p);
        });
    }
}

fn main() {}