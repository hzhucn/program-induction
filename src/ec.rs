use std::collections::HashMap;

use super::{Expression, Task, DSL};

/// The entry point for one iteration of the EC algorithm.
///
/// Lots of type variables here.
///
/// - `S` is some "state" initially passed in as a prior, probably taking the form of production
///   probabilities. But it's ultimately up to you what it is and how it gets used. The important
///   thing is that it's something you can change during ec at any point in the pipeline (which
///   will probably be just compression updating production probabilities and adding to the DSL).
/// - `O` is the observation type, something which the recognizer/explorer can take advantage of
///   instead of just basing search off of the type and oracle response.
/// - `RS` is something the recognizer returns that the explorer can use. This could be, for
///   example, a map from task number to a set of production probabilities.
/// - `R`, `E`, and `C` are the types, described in the `where` clause of the function signature
///   here, for a recognizer, an explorer, and a compressor respectively. Note that the compressor
///   returns a DSL as well as the best solution for each task.
pub fn ec<S, O, RS, R, E, C>(
    prior: &DSL,
    mut state: &mut S,
    tasks: &Vec<Task<O>>,
    recognizer: Option<R>,
    explore: E,
    compress: C,
) -> (DSL, Vec<Option<Expression>>)
where
    R: FnOnce(&DSL, &mut S, &Vec<Task<O>>) -> RS,
    E: FnOnce(&DSL, &mut S, &Vec<Task<O>>, Option<RS>) -> HashMap<usize, Vec<Expression>>,
    C: FnOnce(&DSL, &mut S, &Vec<Task<O>>, HashMap<usize, Vec<Expression>>)
        -> (DSL, Vec<Option<Expression>>),
{
    let recognized = recognizer.map(|r| r(prior, &mut state, tasks));
    let frontiers = explore(prior, &mut state, tasks, recognized);
    compress(prior, &mut state, tasks, frontiers)
}