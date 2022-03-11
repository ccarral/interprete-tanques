use std::collections::HashMap;

use crate::error::ErrorInterprete;
use crate::parser::*;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

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
        match pair.as_rule() {
            Rule::inst => {
                let inst_inner = pair.into_inner().next().unwrap();
                self.parse_node(inst_inner)?;
            }
            Rule::decl => {
                let mut decl_pairs = pair.into_inner();
                let var_name = decl_pairs.next().unwrap().as_str();
                let expr = decl_pairs.next().unwrap();
                self.parse_node(expr)?;
                let valor = self.expr_stack.pop().unwrap();
                self.vars.insert(var_name.into(), valor);
            }
            Rule::expr => {
                let expr_inner = pair.into_inner().next().unwrap();
                // dbg!(&expr_inner);
                self.parse_node(expr_inner)?;
            }
            Rule::expr_par => {
                let expr_par_inner = pair.into_inner().next().unwrap();
                self.parse_node(expr_par_inner)?;
            }
            Rule::int => {
                dbg!(&pair);
                let valor: isize = pair.as_str().parse().unwrap();
                self.expr_stack.push(valor);
            }

            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn step_inst(&mut self) {
        if let Some(pair) = self.pairs.next() {
            dbg!(&pair);
            self.parse_node(pair);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_var_decl() {
        let mut interprete = Interprete::new("var x =( 1 );var y = (-33) ;").unwrap();
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("x"), Some(&1));
        assert_eq!(interprete.get_var_value("y"), None);
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("y"), Some(&-33));
    }
}
