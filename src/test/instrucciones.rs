use crate::interprete::{eval_logic, Interpreter};
use crate::parser::ParserTanques;
use crate::parser::*;
use crate::scope::Scope;
use pest::Parser;

#[test]
pub fn test_expr() {
    let mut interprete = Interpreter::new(
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
    let mut interprete = Interpreter::new("var x = 1;x = x + 1;").unwrap();
    interprete.step_inst().unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(1));
    interprete.step_inst().unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(2));
}

#[test]
fn test_si() {
    let mut interprete =
        Interpreter::new("var x = 1; si(x == 1){ x = x + 2; x = x + 4; } var y = 2; x = 10;")
            .unwrap();
    interprete.step_inst().unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(1));

    interprete.step_inst().unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(3));

    interprete.step_inst().unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(7));

    interprete.step_inst().unwrap();
    assert_eq!(interprete.get_var_value("y"), Some(2));
    assert_eq!(interprete.get_var_value("x"), Some(1));

    interprete.step_inst().unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(10));
}
