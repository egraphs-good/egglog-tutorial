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

use crate::ast::Type;

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

    // Problem  1: run the egglog program in defn.egg
    let mut egraph = new_experimental_egraph();
    egraph
        .parse_and_run_program(Some("defn.egg".to_string()), egglog_program())
        .unwrap();

    // Problem  2:
    //
    // For each variable in the declaration, bind the variable x to
    // a corresponding (MVar x) and (SVar x)
    //
    // For each matrix declaration, additionally insert its dimension
    // information into the "MatrixDim" relation in the E-graph:
    //
    //    (relation MatrixDim (String i64 i64))
    //
    for decl in core_bindings.declares.iter() {
        let x = &decl.var;
        if let Type::Matrix { nrows, ncols } = decl.ty {
            let program = format!(
                "(MatrixDim \"{x}\" {nrows} {ncols})
                (let {x} (MVar \"{x}\"))"
            );
            egraph.parse_and_run_program(None, &program).unwrap();
        } else {
            let program = format!("(let {x} (SVar \"{x}\"))");
            egraph.parse_and_run_program(None, &program).unwrap();
        }
    }

    // Problem  3:
    //
    // For each binding, bind the variable x to its corresponding definitions.
    //
    // We have provided [`to_egglog_expr`] function that converts a [`CoreExpr`]
    // to an egglog expression [`egglog::ast::Expr`]
    for bind in core_bindings.bindings.iter() {
        let var = &bind.var;
        let expr = &bind.expr;
        let expr = to_egglog_expr(expr);
        let action = Action::Let(span!(), var.to_string(), expr);

        egraph.run_program(vec![Command::Action(action)]).unwrap();
    }

    {
        egraph
            .parse_and_run_program(None, "(run-schedule (saturate (run cost-analysis)))")
            .unwrap();
        // Get the cost before optimization
        let output = core_bindings.bindings.last().unwrap();
        let (sort, value) = egraph.eval_expr(&var(&output.var)).unwrap();
        let cost_model = DynamicCostModel;
        let (_termdag, _term, cost) = egraph
            .extract_value_with_cost_model(&sort, value, cost_model)
            .unwrap();
        eprintln!("Cost before optimization: {cost}");
    }

    // Problem  4:
    //
    // Now we have inserted all the ASTs and definitions. We will run our rules.
    // To start with, let's run our rules 20 times (`(run 20)`).

    // egraph.parse_and_run_program(None, "(run 20)").unwrap();

    add_scheduler_builder("first-n".into(), Box::new(new_first_n_scheduler));
    let schedule = "
    (run-schedule 
      (let-scheduler first-100 (first-n 100))
      (repeat 100 (run-with first-100 optimization))
      (saturate (run cost-analysis))
    )
    ";
    egraph.parse_and_run_program(None, schedule).unwrap();

    // Problem  5:
    //
    // Extract the optimized program using the `DynamicCostModel` from egglog_experimental.
    // The extracted program is a directed acyclic graph (DAG) and has type [`TermDag`]` and [`Term`].
    // We have provided the method [`termdag_to_bindings`] to convert to [`CoreBindings`].
    let output = core_bindings.bindings.last().unwrap();
    let (sort, value) = egraph.eval_expr(&var(&output.var)).unwrap();
    let cost_model = DynamicCostModel;
    // let cost_model = AstDepthCostModel;
    let (termdag, term, cost) = egraph
        .extract_value_with_cost_model(&sort, value, cost_model)
        .unwrap();
    eprintln!("Cost after optimization: {cost}");
    let bindings = util::termdag_to_bindings(core_bindings.declares, &termdag, &term);

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
// to the default scheduler, it allows the E-graph grows more gently.
//
// Add this scheduler to egglog_experimental with
//
//     add_scheduler_builder("first-n".into(), Box::new(new_first_n_scheduler));
//
// Update the schedule so that the optimization ruleset uses this scheduler.
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
        if matches.match_size() <= self.n {
            matches.choose_all();
        } else {
            for i in 0..self.n {
                matches.choose(i);
            }
        }
        matches.match_size() < self.n * 2
    }
}

// Problem  8:
//
// We are going to define an alternative cost model that is not "sum of node costs".
//
// The cost model, named [`AstDepthCostModel`] assigns the depth of an AST as its cost,
// so an extractor using this cost model will extract a term with the smallest depth.
//
// Use this cost model in our extractor
pub struct AstDepthCostModel;

pub type C = usize;
impl CostModel<C> for AstDepthCostModel {
    fn fold(&self, _head: &str, children_cost: &[C], head_cost: C) -> C {
        children_cost.iter().max().unwrap_or(&0) + head_cost
    }

    fn enode_cost(
        &self,
        _egraph: &EGraph,
        _func: &egglog::Function,
        _row: &egglog::FunctionRow,
    ) -> C {
        1
    }

    fn container_primitive(
        &self,
        _egraph: &EGraph,
        _sort: &egglog::ArcSort,
        _value: egglog::Value,
        element_costs: &[C],
    ) -> C {
        *element_costs.iter().max().unwrap_or(&0)
    }

    fn leaf_primitive(
        &self,
        _egraph: &EGraph,
        _sort: &egglog::ArcSort,
        _value: egglog::Value,
    ) -> C {
        1
    }
}
