use crate::error::ErrorInterprete;
use crate::parser::*;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use std::collections::HashMap;

pub struct Interprete<'a> {
    pairs: Pairs<'a, Rule>,
    vars: HashMap<String, isize>,
    expr_stack: Vec<isize>,
}

impl<'a> Interprete<'a> {
    pub fn new(prog: &'a str) -> Result<Self, Error<Rule>> {
        let pairs = ParserTanques::parse(Rule::prog, prog)?;
        for pair in pairs.clone() {
            println!("{pair}");
        }
        let vars = HashMap::new();
        Ok(Self {
            pairs,
            vars,
            expr_stack: Vec::new(),
        })
    }

    pub fn get_var_value(&self, varname: &str) -> Option<&isize> {
        self.vars.get(&varname.to_string())
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
                // self.parse_node(expr)?;
                let valor = eval(expr.into_inner());
                // let valor = self.expr_stack.pop().unwrap();
                self.vars.insert(var_name.into(), valor);
            }
            Rule::expr => {
                let mut pairs = pair.into_inner();
                let expr_pairs = pairs.next().unwrap();

                //Para este punto, ya tenemos la evaluación de la expresión izquierda en el stack
                //Ahora verificamos si hay una operación y operando
            }
            Rule::expr_par => {
                let expr_par_inner = pair.into_inner().next().unwrap();
                self.parse_node(expr_par_inner)?;
            }
            Rule::int => {
                let valor: isize = pair.as_str().parse().unwrap();
                self.expr_stack.push(valor);
            }
            Rule::EOI => {}

            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn step_inst(&mut self) {
        if let Some(pair) = self.pairs.next() {
            self.parse_node(pair);
        }
    }
}

fn eval(expr: Pairs<Rule>) -> isize {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::suma, Assoc::Left) | Operator::new(Rule::resta, Assoc::Left),
    ]);

    let infix = |lhs: isize, op: Pair<Rule>, rhs: isize| match op.as_rule() {
        Rule::suma => lhs + rhs,
        Rule::resta => lhs - rhs,
        _ => unreachable!(),
    };

    let primary = |pair: Pair<Rule>| match pair.as_rule() {
        Rule::expr_par => {
            let expr_inner = pair.into_inner();
            eval(expr_inner)
        }
        Rule::expr => eval(pair.into_inner()),
        Rule::int => pair.as_str().parse::<isize>().unwrap(),
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
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("x"), Some(&1));
        assert_eq!(interprete.get_var_value("y"), None);
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("y"), Some(&-33));
    }

    #[test]
    pub fn test_expr() {
        // let mut interprete = Interprete::new("var x = 1  + 2; var y = 1 + 2 - 3").unwrap();
        let mut interprete = Interprete::new("var x = 1 + 2;var y = 1 - 2;var z = 1-2+3;").unwrap();
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("x"), Some(&3));
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("y"), Some(&-1));
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("z"), Some(&2));
    }
}
