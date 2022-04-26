use crate::error::ErrorInterprete;
use crate::parser::*;
use crate::scope::Scope;
use crate::tank_status::{TankDirection, TankStatus};
use pest::error::{Error, LineColLocation};
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExecutionContext<'a> {
    Block,
    IfBlock,
    While(Pair<'a, Rule>),
}

pub struct Interpreter<'a> {
    exec_stack: Vec<(Pairs<'a, Rule>, ExecutionContext<'a>)>,
    scope: Scope,
}

impl<'a> Interpreter<'a> {
    pub fn new(prog: &'a str) -> Result<Self, LineColLocation> {
        let pairs = ParserTanques::parse(Rule::prog, prog).map_err(|e| e.line_col)?;
        let scope = Scope::new();
        Ok(Self {
            exec_stack: vec![(pairs, ExecutionContext::Block)],
            scope,
        })
    }

    pub fn get_var_value(&self, varname: &str) -> Option<isize> {
        self.scope.get_var_value(&varname)
    }

    fn parse_node(
        &mut self,
        pair: Pair<'a, Rule>,
        current_status: &TankStatus,
    ) -> Result<TankStatus, ErrorInterprete> {
        println!("Descending");
        dbg!(&pair.as_rule());
        match pair.as_rule() {
            Rule::inst => {
                let inst_inner = pair.into_inner().next().unwrap();
                self.parse_node(inst_inner, current_status)
            }
            Rule::bloque => {
                let bloque_inner = pair.into_inner().next().unwrap();
                self.parse_node(bloque_inner, current_status)
            }
            Rule::decl => {
                let mut decl_pairs = pair.into_inner();
                let var_name = decl_pairs.next().unwrap().as_str();
                let expr = decl_pairs.next().unwrap();
                let valor = eval(expr.into_inner(), &self.scope)?;
                self.scope.set_var(var_name.into(), valor);
                Ok(*current_status)
            }
            Rule::asig => {
                let mut asig_pairs = pair.into_inner();
                let var_name = asig_pairs.next().unwrap().as_str();
                let expr = asig_pairs.next().unwrap();
                let valor = eval(expr.into_inner(), &self.scope)?;
                self.scope.set_var(var_name.into(), valor);
                Ok(*current_status)
            }
            Rule::bloque_si => {
                let mut pairs = pair.into_inner();
                let expr_logic = pairs.next().unwrap();
                let expr_val = eval_logic(expr_logic.into_inner(), &self.scope)?;
                if expr_val {
                    self.scope.add();
                    // Debería de tener el bloque
                    let instrucciones = pairs.next().unwrap().into_inner();
                    self.exec_stack
                        .push((instrucciones, ExecutionContext::IfBlock));
                    self.step_inst(current_status)
                } else {
                    Ok(*current_status)
                }
            }
            Rule::bloque_mientras => {
                let mut pairs = pair.into_inner();
                let expr_logic = pairs.next().unwrap();
                let expr_logic_clone = expr_logic.clone();
                let expr_val = eval_logic(expr_logic.into_inner(), &self.scope)?;
                if expr_val {
                    self.scope.add();
                    let instrucciones = pairs.next().unwrap().into_inner();
                    let instrucciones_clone = instrucciones.clone();
                    self.exec_stack.push((
                        instrucciones,
                        ExecutionContext::While(expr_logic_clone.clone()),
                    ));
                    self.exec_stack.push((
                        instrucciones_clone,
                        ExecutionContext::While(expr_logic_clone),
                    ));
                    self.step_inst(current_status)
                } else {
                    Ok(*current_status)
                }
            }
            Rule::gira => {
                let mut pairs = pair.into_inner();
                // let dir = pairs.skip(1).next().unwrap();
                let dir = pairs.next().unwrap();
                let new_dir = match dir.as_str() {
                    "izquierda" => match current_status.get_dir() {
                        TankDirection::North => TankDirection::West,
                        TankDirection::West => TankDirection::South,
                        TankDirection::South => TankDirection::East,
                        TankDirection::East => TankDirection::North,
                    },
                    "derecha" => match current_status.get_dir() {
                        TankDirection::North => TankDirection::East,
                        TankDirection::West => TankDirection::North,
                        TankDirection::South => TankDirection::West,
                        TankDirection::East => TankDirection::South,
                    },
                    _ => unreachable!(),
                };

                let mut new_status = *current_status;
                new_status.set_dir(new_dir);
                Ok(new_status)
            }
            Rule::avanza => {
                let (old_x, old_y) = current_status.get_pos();
                let (new_x, new_y) = match current_status.get_dir() {
                    TankDirection::North => (old_x, old_y.saturating_sub(1)),
                    TankDirection::West => (old_x.saturating_sub(1), old_y),
                    TankDirection::South => (
                        old_x,
                        if old_y + 1 == TankStatus::GRID_DIMMENSIONS {
                            old_y
                        } else {
                            old_y + 1
                        },
                    ),
                    TankDirection::East => (
                        if old_x + 1 == TankStatus::GRID_DIMMENSIONS {
                            old_x
                        } else {
                            old_x + 1
                        },
                        old_y,
                    ),
                };
                let mut new_status = *current_status;
                new_status.set_pos(new_x, new_y);
                Ok(new_status)
            }
            Rule::EOI => Ok(*current_status),
            _ => unreachable!(),
        }
    }

    pub fn step_inst(
        &mut self,
        current_status: &TankStatus,
    ) -> Result<TankStatus, ErrorInterprete> {
        let (mut current_exec_block, ctx) = self.exec_stack.pop().unwrap();
        if let Some(pair) = dbg!(current_exec_block.next()) {
            self.exec_stack.push((current_exec_block, ctx));
            self.parse_node(pair, current_status)
        } else {
            // Reached the end of the iterator, check for condition
            match ctx {
                ExecutionContext::Block => Ok(TankStatus::default()),
                ExecutionContext::IfBlock => {
                    self.scope.drop();
                    self.step_inst(current_status)
                }
                ExecutionContext::While(p) => {
                    let pair = p.clone();
                    let pairs = pair.into_inner();
                    let expr_val = dbg!(eval_logic(pairs, &self.scope))?;
                    if !expr_val {
                        // Loop ends, pop the cloned pairs object
                        self.exec_stack.pop();
                        self.step_inst(current_status)
                    } else {
                        // Loop  continues, push another pairs object to the stack
                        let (pairs, ctx) = self.exec_stack.pop().unwrap();
                        let pairs_clone = pairs.clone();
                        let ctx_clone = ctx.clone();
                        self.exec_stack.push((pairs, ctx));
                        self.exec_stack.push((pairs_clone, ctx_clone));
                        self.step_inst(current_status)
                    }
                }
            }
        }
    }
}

pub fn eval(expr: Pairs<Rule>, scope: &Scope) -> Result<isize, ErrorInterprete> {
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

pub fn eval_logic(expr: Pairs<Rule>, scope: &Scope) -> Result<bool, ErrorInterprete> {
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
