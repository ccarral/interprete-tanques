use crate::error::ErrorInterprete;
use crate::parser::*;
use crate::scope::Scope;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

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
            Rule::asig => {
                let mut asig_pairs = pair.into_inner();
                let var_name = asig_pairs.next().unwrap().as_str();
                let expr = asig_pairs.next().unwrap();
                let valor = eval(expr.into_inner(), &self.scope)?;
                self.scope.set_var(var_name.into(), valor);
            }
            Rule::bloque_si => {
                let mut pairs = pair.into_inner();
                let expr_logic = pairs.next().unwrap();
                let expr_val = dbg!(eval_logic(expr_logic.into_inner(), &self.scope))?;
                if dbg!(expr_val) {
                    self.scope.add();
                    let instrucciones = pairs.next().unwrap().into_inner();
                    for instruccion in instrucciones {
                        self.parse_node(instruccion)?;
                    }
                }
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

fn eval_logic(expr: Pairs<Rule>, scope: &Scope) -> Result<bool, ErrorInterprete> {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::men, Assoc::Left)
            | Operator::new(Rule::may, Assoc::Left)
            | Operator::new(Rule::men_ig, Assoc::Left)
            | Operator::new(Rule::may_ig, Assoc::Left)
            | Operator::new(Rule::ig, Assoc::Left)
            | Operator::new(Rule::no_ig, Assoc::Left),
        Operator::new(Rule::or, Assoc::Left),
        Operator::new(Rule::and, Assoc::Left),
    ]);

    let infix =
        |lhs: Result<bool, ErrorInterprete>, op: Pair<Rule>, rhs: Result<bool, ErrorInterprete>| {
            match (lhs, rhs) {
                (Ok(lhs), Ok(rhs)) => match op.as_rule() {
                    Rule::or => Ok(lhs || rhs),
                    Rule::and => Ok(lhs && rhs),
                    _ => unreachable!(),
                },
                (e, Ok(_)) => e,
                (Ok(_), e) => e,
                (e, Err(_)) => e,
            }
        };

    let primary = |pair: Pair<Rule>| match pair.as_rule() {
        Rule::expr_par_logic => {
            let expr_inner = pair.into_inner();
            eval_logic(expr_inner, scope)
        }
        Rule::expr_logic => eval_logic(pair.into_inner(), scope),
        Rule::term_logic => eval_logic(pair.into_inner(), scope),
        Rule::comp_logic => {
            let mut pairs = pair.into_inner();
            let lhs = {
                let pairs = pairs.next().unwrap().into_inner();
                eval(pairs, scope)
            }?;
            let op = match pairs.next().unwrap().as_rule() {
                Rule::men => |lhs: isize, rhs: isize| lhs < rhs,
                Rule::men_ig => |lhs: isize, rhs: isize| lhs <= rhs,
                Rule::may => |lhs: isize, rhs: isize| lhs > rhs,
                Rule::may_ig => |lhs: isize, rhs: isize| lhs >= rhs,
                Rule::ig => |lhs: isize, rhs: isize| lhs == rhs,
                Rule::no_ig => |lhs: isize, rhs: isize| lhs != rhs,
                _ => unreachable!(),
            };

            let rhs = {
                let pairs = pairs.next().unwrap().into_inner();
                eval(pairs, scope)
            }?;

            Ok(op(lhs, rhs))
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

    #[test]
    pub fn test_expr_logic() {
        let eval_expr = |expr: &str, scope: &Scope| {
            let pairs = ParserTanques::parse(Rule::expr_logic, expr).unwrap();
            eval_logic(pairs, &scope).unwrap()
        };

        let mut scope = Scope::new();

        let val = eval_expr("2 == 1", &scope);
        assert!(!val);

        let val = eval_expr("1 < 0", &scope);
        assert!(!val);

        let val = eval_expr("4 == 4", &scope);
        assert!(val);

        let val = eval_expr("4 != 3", &scope);
        assert!(val);

        let val = eval_expr("1 < 2 && 6 == 7", &scope);
        assert!(!val);

        let val = eval_expr("1 < 2 && 6 == 6 && 7 != 7", &scope);
        assert!(!val);

        let val = eval_expr("1 < 2 || 6 == 6 && 7 != 7", &scope);
        assert!(val);

        scope.set_var("x", 8);
        let val = eval_expr("x == 8", &scope);
        assert!(val);

        let val = eval_expr("x != 8", &scope);
        assert!(!val);

        scope.set_var("y", 9);
        let val = eval_expr("x < y", &scope);
        assert!(val);

        let val = eval_expr("x == y", &scope);
        assert!(!val);
    }

    #[test]
    fn test_asig() {
        let mut interprete = Interprete::new("var x = 1;x = x + 1;").unwrap();
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("x"), Some(1));
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("x"), Some(2));
    }

    #[test]
    fn test_si() {
        let mut interprete = Interprete::new("var x = 1; si(x != 0){ x = x + 2; }").unwrap();
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("x"), Some(1));
        interprete.step_inst().unwrap();
        assert_eq!(interprete.get_var_value("x"), Some(3));
    }
}
