use std::collections::HashMap;

use crate::parser::*;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

pub struct Interprete<'a> {
    pairs: Pairs<'a, Rule>,
    vars: HashMap<String, isize>,
}

impl<'a> Interprete<'a> {
    pub fn new(prog: &'a str) -> Result<Self, Error<Rule>> {
        let pairs = ParserTanques::parse(Rule::prog, prog)?;
        for pair in pairs.clone() {
            println!("{pair}");
        }
        let vars = HashMap::new();
        Ok(Self { pairs, vars })
    }

    pub fn get_var_value(&self, varname: &str) -> Option<&isize> {
        self.vars.get(&varname.to_string())
    }

    fn parse_node(&mut self, pair: Pair<Rule>) {
        match pair.as_rule() {
            Rule::inst => {
                let inst_inner = pair.into_inner().next().unwrap();
                self.parse_node(inst_inner);
            }
            Rule::decl => {
                let mut decl_pairs = pair.into_inner();

                let var_name = decl_pairs.next().unwrap().as_str();
                let valor: isize = decl_pairs.next().unwrap().as_str().parse().unwrap();

                self.vars.insert(var_name.into(), valor);
            }

            _ => unreachable!(),
        }
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
        let mut interprete = Interprete::new("var x = 1;var y =-33;").unwrap();
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("x"), Some(&1));
        assert_eq!(interprete.get_var_value("y"), None);
        interprete.step_inst();
        assert_eq!(interprete.get_var_value("y"), Some(&-33));
    }
}
