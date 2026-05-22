#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::io::Read;

use egglog_experimental::ast::{Action, Command};
use egglog_experimental::extract::{CostModel, Extractor};
use egglog_experimental::scheduler::{Matches, Scheduler};
use egglog_experimental::{
    self as egglog,
    DynamicCostModel, TermDag,
    ast::Literal,
    new_experimental_egraph,
    prelude::{exprs::*, *},
};

use crate::ast::{CoreBindings, Type};

mod ast;
mod util;
use crate::util::*;

fn egglog_program() -> &'static str {
    include_str!("defn.egg")
}

fn main() {
    // Read from stdin
    let mut program = String::new();
    std::io::stdin().read_to_string(&mut program).unwrap();

    // Parse to [`CoreBindings`]
    let bindings = ast::grammar::BindingsParser::new()
        .parse(&program)
        .expect("parsing bindings");
    let core_bindings = match bindings.lower() {
        Ok(bindings) => bindings,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    // Problem  1: run defn.egg
    //
    // Create a new experimental EGraph with `new_experimental_egraph()` and run
    // the egglog program from defn.egg using
    //     `egraph.parse_and_run_program(None, program)`.
    let mut egraph = new_experimental_egraph();
    todo!("Problem 1");

    // Problem  2:
    //
    // For each variable declaration in the input program,
    // bind the egglog variable x to either (MVar x) or (SVar x)
    //
    // For each matrix declaration, additionally insert its dimension
    // information into the "MatrixDim" relation in the E-graph:
    //
    //    (relation MatrixDim (String i64 i64))
    //
    // Hint: use `egraph.parse_and_run_program(None, &format!("..."))`.
    for decl in core_bindings.declares.iter() {
        let x = &decl.var;
        if let Type::Matrix { nrows, ncols } = decl.ty {
            todo!("Problem 2")
        } else {
            todo!("Problem 2")
        }
    }

    // Problem  3:
    //
    // For each variable assignment in the input program,
    // bind the egglog variable to its corresponding expression.
    //
    // We have provided [`to_egglog_expr`] in util.rs that converts a [`CoreExpr`]
    // to an egglog AST expression [`egglog::ast::Expr`].
    //
    // To run a `let` command, build a
    //     Command::Action(Action::Let(span!(), name, expr))
    // and pass it to `egraph.run_program(vec![cmd])`.
    for bind in core_bindings.bindings.iter() {
        let var = &bind.var;
        let expr = &bind.expr;

        todo!("Problem 3")
    }

    // Problem  4:
    //
    // Now we have inserted all the ASTs and definitions. We will run our rules.
    // To start with, let's run our rules 20 times with `(run 20)`:
    //
    //     egraph.parse_and_run_program(None, "(run 20)").unwrap();

    todo!("Problem 4");

    // Problem  5:
    //
    // Extract the optimized program using `DynamicCostModel` and `Extractor`.
    //
    // Steps:
    //   a) Evaluate the output variable to get its (ArcSort, Value):
    //        egraph.eval_expr(&exprs::var(&output.var))
    //   b) Compute an extractor:
    //        Extractor::compute_costs_from_rootsorts(Some(vec![sort]), &egraph, DynamicCostModel)
    //   c) Extract the best term:
    //        extractor.extract_best(&egraph, &mut termdag, value)
    //   d) Convert to CoreBindings using `termdag_to_bindings`
    let output = core_bindings.bindings.last().unwrap();
    let bindings: CoreBindings = todo!("Problem 5");

    // Print the optimized bindings
    println!("{bindings}");

    // Problem  6:
    //
    // Break down the rules into optimization rules and analysis rules by adding
    // `:ruleset optimization` and `:ruleset analysis` annotations in defn.egg,
    // and declaring the rulesets:
    //
    //     (ruleset optimization)
    //     (ruleset analysis)
    //
    // Then replace `(run 20)` above with a schedule that:
    //   - Saturates the analysis rules before each optimization step
    //   - Runs the optimization rules once per iteration
    //
    // Hint: use `(run-schedule (repeat 20 (saturate analysis) (run optimization)))`.
    //
    // Also update `egglog_program()` to include your updated defn file.

    // Problem  7:
    //
    // Fill in the blanks for the [`FirstNScheduler`] below. FirstNScheduler
    // applies at most `n` matches of a rule in each iteration. Compared
    // to the default scheduler, it allows the E-graph to grow more gently.
    //
    // Register the scheduler with the egraph:
    //
    //     let scheduler_id = egraph.add_scheduler(Box::new(FirstNScheduler { n: 3 }));
    //
    // Then use `egraph.step_rules_with_scheduler(scheduler_id, "optimization")`
    // inside a loop to run the optimization ruleset with the scheduler.
    //
    // Update Problem 6's schedule to use this scheduler for optimization rules.
}

// Problem  7:
//
// Implement the `filter_matches` method.
// FirstNScheduler should apply at most `n` matches of a rule per iteration:
//   - If there are <= n matches: apply all of them, return false (done).
//   - If there are > n matches: apply the first n, return true (more work remains).
#[derive(Clone)]
struct FirstNScheduler {
    n: usize,
}

impl Scheduler for FirstNScheduler {
    fn filter_matches(&mut self, _rule: &str, _ruleset: &str, matches: &mut Matches) -> bool {
        todo!("Problem 7")
    }
}

// Problem  8:
//
// We are going to define an alternative cost model that assigns the *depth*
// of an AST as its cost. An extractor using this model will prefer shallower terms.
//
// The cost of a compound node is: max(children_costs) + enode_cost
// The cost of a leaf (primitive) is: 0
//
// Use this cost model in the extractor from Problem 5 instead of DynamicCostModel.
pub struct AstDepthCostModel;

pub type C = usize;
impl CostModel<C> for AstDepthCostModel {
    fn fold(&self, _head: &str, children_cost: &[C], head_cost: C) -> C {
        todo!("Problem 8")
    }

    fn enode_cost(
        &self,
        _egraph: &EGraph,
        _func: &egglog::Function,
        _row: &egglog::FunctionRow,
    ) -> C {
        todo!("Problem 8")
    }

    fn container_cost(
        &self,
        _egraph: &EGraph,
        _sort: &egglog::ArcSort,
        _value: egglog::Value,
        element_costs: &[C],
    ) -> C {
        todo!("Problem 8")
    }

    fn base_value_cost(
        &self,
        _egraph: &EGraph,
        _sort: &egglog::ArcSort,
        _value: egglog::Value,
    ) -> C {
        todo!("Problem 8")
    }
}
