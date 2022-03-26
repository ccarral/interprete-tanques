use crate::error::ErrorInterprete;
use crate::parser::*;
use crate::scope::Scope;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use std::collections::HashMap;

pub struct Interprete<'a> {
    pairs: Pairs<'a, Rule>,
    scope: Scope,
}

impl<'a> Interprete<'a> {
    pub fn new(prog: &'a str) -> Result<Self, Error<Rule>> {
        let pairs = ParserTanques::parse(Rule::prog, prog)?;
        for pair in pairs.clone() {
            println!("{pair}");
        }
        let scope = Scope::new();
        Ok(Self { pairs, scope })
    }

    pub fn get_var_value(&self, varname: &str) -> Option<isize> {
        self.scope.get_var_value(&varname)
    }

    fn parse_node(&mut self, pair: Pair<Rule>) -> Result<(), ErrorInterprete> {
        println!("Descending");
        dbg!(&pair.as_rule());
        match pair.as_rule() {
            Rule::inst => {
                let inst_inner = pair.into_inner().next().unwrap();
                self.parse_node(inst_inner)?;
            }
            Rule::decl => {
                let mut decl_pairs = pair.into_inner();
                let var_name = decl_pairs.next().unwrap().as_str();
                let expr = decl_pairs.next().unwrap();
                let valor = eval(expr.into_inner(), &self.scope)?;
                self.scope.set_var(var_name.into(), valor);
            }
            Rule::expr_par => {
                let expr_par_inner = pair.into_inner().next().unwrap();
                self.parse_node(expr_par_inner)?;
            }

            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn step_inst(&mut self) -> Result<(), ErrorInterprete> {
        if let Some(pair) = self.pairs.next() {
            self.parse_node(pair)?;
        }
        Ok(())
    }
}

fn eval(expr: Pairs<Rule>, scope: &Scope) -> Result<isize, ErrorInterprete> {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::suma, Assoc::Left) | Operator::new(Rule::resta, Assoc::Left),
        Operator::new(Rule::mult, Assoc::Left) | Operator::new(Rule::div, Assoc::Left),
    ]);

    let infix = |lhs: Result<isize, ErrorInterprete>,
                 op: Pair<Rule>,
                 rhs: Result<isize, ErrorInterprete>| {
        match (lhs, rhs) {
            (Ok(lhs), Ok(rhs)) => match op.as_rule() {
                Rule::suma => Ok(lhs + rhs),
                Rule::resta => Ok(lhs - rhs),
                Rule::mult => Ok(lhs * rhs),
                Rule::div => Ok(lhs / rhs),
                _ => unreachable!(),
            },
            (e, Ok(_)) => e,
            (Ok(_), e) => e,
            (e, Err(_)) => e,
        }
    };

    let primary = |pair: Pair<Rule>| match pair.as_rule() {
        Rule::expr_par => {
            let expr_inner = pair.into_inner();
            eval(expr_inner, scope)
        }
        Rule::expr => eval(pair.into_inner(), scope),
        Rule::int => Ok(pair.as_str().parse::<isize>().unwrap()),
        Rule::nom_var => match scope.get_var_value(pair.as_str()) {
            Some(value) => Ok(value),
            None => Err(ErrorInterprete::VarNoDecl(pair.as_str().into())),
        },
        r => {
            dbg!(r);
            unreachable!()
        }
    };

    climber.climb(expr, primary, infix)
}

fn eval_logic(expr: Pairs<Rule>) -> bool {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::men, Assoc::Left)
            | Operator::new(Rule::may, Assoc::Left)
            | Operator::new(Rule::men_ig, Assoc::Left)
            | Operator::new(Rule::may_ig, Assoc::Left)
            | Operator::new(Rule::ig, Assoc::Left),
    ]);

    let infix = |lhs: bool, op: Pair<Rule>, rhs: bool| match op.as_rule() {
        _ => unreachable!(),
    };

    let primary = |pair: Pair<Rule>| match pair.as_rule() {
        Rule::expr_par_logic => {
            let expr_inner = pair.into_inner();
            eval_logic(expr_inner)
        }
        Rule::expr_logic => eval_logic(pair.into_inner()),
        Rule::term_logic => eval_logic(pair.into_inner()),
        Rule::comp_logic => {
            let mut pairs = pair.into_inner();
            let lhs = pairs.next().unwrap();
            let op = pairs.next().unwrap().as_rule();
            let rhs = pairs.next().unwrap().as_rule();
            true
        }
        r => {
            dbg!(r);
            unreachable!()
        }
    };

    climber.climb(expr, primary, infix)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_var_decl() {
        let mut interprete = Interprete::new("var x =( 1 );var y = -33 ;").unwrap();
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("x"), Some(1));
        assert_eq!(interprete.get_var_value("y"), None);
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("y"), Some(-33));
    }

    #[test]
    pub fn test_expr() {
        let mut interprete = Interprete::new(
            "var x = 1 + 2;var y = 1 - 2 ;var z = 4 * 2; var w = 4/(2*2); var a = w + 10; var b = n + 1;",
        )
        .unwrap();
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("x"), Some(3));
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("y"), Some(-1));
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("z"), Some(8));
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("w"), Some(1));
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("a"), Some(11));
        let res = interprete.step_inst();
        assert!(res.is_err());
    }
}
