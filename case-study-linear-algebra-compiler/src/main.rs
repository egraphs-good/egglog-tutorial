#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::io::Read;

use egglog_experimental::ast::Command;
use egglog_experimental::scheduler::Matches;
use egglog_experimental::{self as egglog, add_scheduler_builder, DynamicCostModel};
use egglog_experimental::{
    ast::Literal,
    extract::CostModel,
    new_experimental_egraph,
    prelude::{exprs::*, *},
    scheduler::Scheduler,
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
    // bind the egglog variable x to its corresponding expression.
    //
    // We have provided [`to_egglog_expr`] function that converts a [`CoreExpr`]
    // to an egglog expression [`egglog::ast::Expr`]
    for bind in core_bindings.bindings.iter() {
        let var = &bind.var;
        let expr = &bind.expr;
        
        todo!("Problem 3")
    }

    // Problem  4:
    //
    // Now we have inserted all the ASTs and definitions. We will run our rules.
    // To start with, let's run our rules 20 times (`(run 20)`).

    todo!("Problem 4");

    // Problem  5:
    //
    // Extract the optimized program using the `DynamicCostModel` from egglog_experimental.
    // The extracted program is a directed acyclic graph (DAG) and has type [`TermDag`]` and [`Term`].
    // We have provided the method [`termdag_to_bindings`] to convert to [`CoreBindings`].
    let output = core_bindings.bindings.last().unwrap();
    let bindings: CoreBindings = todo!("Problem 5");

    // Print the optimized bindings
    println!("{bindings}");

    // Problem  6:
    //
    // Break down the rules into optimization rules and analysis rules.
    // Improve the schedule above with the more refined rulesets.
}

// Problem  7:
//
// Fill in the blanks for the [`FirstNScheduler`]. FirstNScheduler
// applies at most `n` matches of a rule in each iteration. Compared
// to the default scheduler, it allows the E-graph to grow more gently.
//
// Add this scheduler to the global scheduler list in egglog-experimental with
//
//     add_scheduler_builder("first-n".into(), Box::new(new_first_n_scheduler));
//
// Update the `run-schedule` so that the optimization ruleset uses this scheduler.
pub fn new_first_n_scheduler(_egraph: &EGraph, exprs: &[egglog::ast::Expr]) -> Box<dyn Scheduler> {
    assert!(exprs.len() == 1);
    let egglog::ast::Expr::Lit(_, Literal::Int(n)) = exprs[0] else {
        panic!("wrong arguments to first n scheduler");
    };
    Box::new(FirstNScheduler { n: n as usize })
}

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
// We are going to define an alternative cost model that is not "sum of node costs".
//
// The cost model, named [`AstDepthCostModel`] assigns the depth of an AST as its cost,
// so an extractor using this cost model will extract a term with the smallest depth.
//
// Use this cost model in our extractor.
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

    fn container_primitive(
        &self,
        _egraph: &EGraph,
        _sort: &egglog::ArcSort,
        _value: egglog::Value,
        element_costs: &[C],
    ) -> C {
        todo!("Problem 8")
    }

    fn leaf_primitive(
        &self,
        _egraph: &EGraph,
        _sort: &egglog::ArcSort,
        _value: egglog::Value,
    ) -> C {
        todo!("Problem 8")
    }
}
