use crate::ast;
use egglog_experimental::ast::Literal;
use egglog_experimental::prelude::exprs::{self, call, var};
use egglog_experimental::util::IndexMap;
use egglog_experimental::{self as egglog, Term, TermDag};

pub fn to_egglog_expr(expr: &ast::CoreExpr) -> egglog::ast::Expr {
    match expr {
        ast::CoreExpr::SVar(name) | ast::CoreExpr::MVar(name) => var(name),
        ast::CoreExpr::Num(value) => call("Num", vec![exprs::int(*value)]),
        ast::CoreExpr::SAdd(left, right)
        | ast::CoreExpr::SMul(left, right)
        | ast::CoreExpr::MAdd(left, right)
        | ast::CoreExpr::MMul(left, right)
        | ast::CoreExpr::Scale(left, right)
        | ast::CoreExpr::SSub(left, right)
        | ast::CoreExpr::SDiv(left, right) => {
            let left_expr = to_egglog_expr(left);
            let right_expr = to_egglog_expr(right);
            let constructor = constructor_to_string(expr);
            call(constructor, vec![left_expr, right_expr])
        }
    }
}

fn constructor_to_string(constructor: &ast::CoreExpr) -> &str {
    match constructor {
        ast::CoreExpr::SAdd(..) => "SAdd",
        ast::CoreExpr::SMul(..) => "SMul",
        ast::CoreExpr::MAdd(..) => "MAdd",
        ast::CoreExpr::MMul(..) => "MMul",
        ast::CoreExpr::Scale(..) => "Scale",
        ast::CoreExpr::SSub(..) => "SSub",
        ast::CoreExpr::SDiv(..) => "SDiv",
        _ => unreachable!("binary constructor expected"),
    }
}

fn string_to_binary_constructor(
    constructor: &str,
    l: ast::CoreExpr,
    r: ast::CoreExpr,
) -> ast::CoreExpr {
    match constructor {
        "SAdd" => ast::CoreExpr::SAdd(Box::new(l), Box::new(r)),
        "SMul" => ast::CoreExpr::SMul(Box::new(l), Box::new(r)),
        "MAdd" => ast::CoreExpr::MAdd(Box::new(l), Box::new(r)),
        "MMul" => ast::CoreExpr::MMul(Box::new(l), Box::new(r)),
        "Scale" => ast::CoreExpr::Scale(Box::new(l), Box::new(r)),
        "SSub" => ast::CoreExpr::SSub(Box::new(l), Box::new(r)),
        "SDiv" => ast::CoreExpr::SDiv(Box::new(l), Box::new(r)),
        _ => panic!("Invalid constructor: {}", constructor),
    }
}

pub fn termdag_to_bindings(
    declares: Vec<ast::Declare>,
    termdag: &TermDag,
    term: &Term,
) -> ast::CoreBindings {
    fn process_term(
        termdag: &TermDag,
        term: &Term,
        bindings: &mut IndexMap<String, ast::CoreExpr>,
        name: String,
    ) -> String {
        if bindings.contains_key(&name) {
            return name;
        }
        match term {
            Term::App(op, args) => match op.as_str() {
                "SAdd" | "SMul" | "SSub" | "SDiv" | "Scale" | "MAdd" | "MMul" => {
                    let left = termdag.get(args[0]);
                    let lvar = process_term(termdag, left, bindings, format!("v{}", args[0]));
                    let right = termdag.get(args[1]);
                    let rvar = process_term(termdag, right, bindings, format!("v{}", args[1]));
                    let (lchild, rchild) = if op.as_str() == "Scale" {
                        (ast::CoreExpr::SVar(lvar), ast::CoreExpr::MVar(rvar))
                    } else if op.as_str().starts_with("S") {
                        (ast::CoreExpr::SVar(lvar), ast::CoreExpr::SVar(rvar))
                    } else {
                        (ast::CoreExpr::MVar(lvar), ast::CoreExpr::MVar(rvar))
                    };
                    let expr = string_to_binary_constructor(op, lchild, rchild);

                    bindings.insert(name.to_string(), expr);
                    name
                }
                "MVar" | "SVar" => {
                    let arg = termdag.get(args[0]);
                    let Term::Lit(Literal::String(name)) = arg else {
                        unreachable!()
                    };
                    name.to_string()
                }
                "Num" => {
                    let arg = termdag.get(args[0]);
                    let Term::Lit(Literal::Int(value)) = arg else {
                        unreachable!()
                    };
                    let expr = ast::CoreExpr::Num(*value);
                    bindings.insert(name.to_string(), expr);
                    name
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    let mut bindings: IndexMap<String, ast::CoreExpr> = Default::default();
    process_term(termdag, term, &mut bindings, "e".into());
    let bindings = bindings
        .into_iter()
        .map(|(name, expr)| ast::CoreBinding { var: name, expr })
        .collect();
    ast::CoreBindings { bindings, declares }
}
