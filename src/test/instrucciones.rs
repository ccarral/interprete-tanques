use crate::interprete::{eval_logic, Interpreter};
use crate::parser::ParserTanques;
use crate::parser::*;
use crate::scope::Scope;
use crate::tank_status::{Position, TankDirection, TankStatus, GRID_DIMMENSIONS};
use pest::Parser;

#[test]
pub fn test_expr() {
    let mut interprete = Interpreter::new(
            "var x = 1 + 2;var y = 1 - 2 ;var z = 4 * 2; var w = 4/(2*2); var a = w + 10; var b = n + 1;",
        )
        .unwrap();

    let status = TankStatus::default();
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(3));
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("y"), Some(-1));
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("z"), Some(8));
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("w"), Some(1));
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("a"), Some(11));
    let res = interprete.step_inst(&status);
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

    let val = eval_expr("1 < 2 || 6 >= 6 && 7 != 7", &scope);
    assert!(val);

    let val = eval_expr("1 <= 2", &scope);
    assert!(val);

    scope.define_new_scope_var("x", 8);
    let val = eval_expr("x == 8", &scope);
    assert!(val);

    let val = eval_expr("x != 8", &scope);
    assert!(!val);

    scope.define_new_scope_var("y", 9);
    let val = eval_expr("x < y", &scope);
    assert!(val);

    let val = eval_expr("x == y", &scope);
    assert!(!val);
}

#[test]
fn test_asig() {
    let mut interprete = Interpreter::new("var x = 1;x = x + 1; y = 2;").unwrap();
    let status = TankStatus::default();
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(1));
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(2));
    let res = interprete.step_inst(&status);
    assert!(res.is_err());
}

#[test]
fn test_si() {
    let mut interprete = Interpreter::new(
        "var x = 1; 
            si(x == 1){ 
                x = x + 2; 
                x = x + 4; 
            } 
            var y = 2; 
            x = 10;",
    )
    .unwrap();
    let status = TankStatus::default();
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(1));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(3));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(7));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("y"), Some(2));
    assert_eq!(interprete.get_var_value("x"), Some(7));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(10));

    let mut interprete = Interpreter::new(
        "var x = 1; 
        si(x == 1){ 
            x = x + 2; 
            si(x <= 3){ 
                x = 100; 
            } 
        } 
        var y = 2; 
        x = 10;",
    )
    .unwrap();

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(1));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(3));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(100));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(100));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(10));

    let mut interprete = Interpreter::new(
        "var x = 2; 
        si(x == 1){ 
            x = x + 2; 
        }otro{
            x = x + 10;
        } 
        var y = 2;",
    )
    .unwrap();
    interprete.step_inst(&status).unwrap();
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(12));

    let mut interprete = Interpreter::new(
        "var x = 1; 
        si(x == 1){ 
            x = x + 2; 
        }otro{
            x = x + 10;
        } 
        var y = 2;",
    )
    .unwrap();
    interprete.step_inst(&status).unwrap();
    interprete.step_inst(&status).unwrap();
    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(3));
}

#[test]
fn test_mientras() {
    let mut interprete = Interpreter::new(
        "var x = 0; 
        mientras(x < 3){ 
            x = x + 1; 
        } 
        var y = 10;",
    )
    .unwrap();
    let status = TankStatus::default();

    assert_eq!(interprete.get_var_value("x"), None);

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(0));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(1));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(2));

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(3));
    assert_eq!(interprete.get_var_value("y"), None);

    interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x"), Some(3));
    assert_eq!(interprete.get_var_value("y"), Some(10));
}

#[test]
fn test_gira() {
    let mut interprete = Interpreter::new(
        "gira derecha;
         gira derecha;
         gira derecha;
         gira derecha;

         gira izquierda;
         gira izquierda;
         gira izquierda;
         gira izquierda;",
    )
    .unwrap();

    let mut status = TankStatus::default();
    status.set_dir(TankDirection::North);

    let new_status = interprete.step_inst(&status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::East);

    let new_status = interprete.step_inst(&new_status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::South);

    let new_status = interprete.step_inst(&new_status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::West);

    let new_status = interprete.step_inst(&new_status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::North);

    let new_status = interprete.step_inst(&new_status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::West);

    let new_status = interprete.step_inst(&new_status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::South);

    let new_status = interprete.step_inst(&new_status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::East);

    let new_status = interprete.step_inst(&new_status).unwrap();
    assert_eq!(new_status.get_dir(), TankDirection::North);
}

#[test]
fn test_dispara() {
    let mut interprete = Interpreter::new("dispara;").unwrap();
    let status = TankStatus::default();
    assert!(!status.shot());
    let new_status = interprete.step_inst(&status).unwrap();
    assert!(new_status.shot())
}

#[test]
fn test_avanza() {
    let mut interprete = Interpreter::new(
        "avanza;
         avanza;
         gira derecha;
         avanza;
         avanza;
         gira derecha;
         avanza;
         avanza;
         gira derecha;
         avanza;
         avanza;
         avanza;
         avanza;

         avanza;
         gira izquierda;
         avanza;
         ",
    )
    .unwrap();

    let mut status = TankStatus::default();
    status.set_dir(TankDirection::East);
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    assert_eq!(status.get_pos(), (2, 0));
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    assert_eq!(status.get_pos(), (2, 2));
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    assert_eq!(status.get_pos(), (0, 2));
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    assert_eq!(status.get_pos(), (0, 0));
    let mut status = interprete.step_inst(&mut status).unwrap();
    let mut status = interprete.step_inst(&mut status).unwrap();
    assert_eq!(status.get_pos(), (0, 0));

    status.set_dir(TankDirection::South);
    status.set_pos(GRID_DIMMENSIONS - 1, GRID_DIMMENSIONS - 1);
    let mut status = interprete.step_inst(&mut status).unwrap();
    assert_eq!(
        status.get_pos(),
        (GRID_DIMMENSIONS - 1, GRID_DIMMENSIONS - 1)
    );
    let mut status = interprete.step_inst(&mut status).unwrap();
    let status = interprete.step_inst(&mut status).unwrap();
    assert_eq!(
        status.get_pos(),
        (GRID_DIMMENSIONS - 1, GRID_DIMMENSIONS - 1)
    );
}

#[test]
fn test_comment() {
    let mut interprete = Interpreter::new(
        "
                avanza; // Esto es un comentario
                // Esto también
                gira derecha;
                // Comentario al final",
    )
    .unwrap();

    let mut tank_status = TankStatus::default();
    tank_status.set_dir(TankDirection::East);
    let tank_status = interprete.step_inst(&tank_status).unwrap();
    assert_eq!(tank_status.get_pos(), (1, 0));
    let tank_status = interprete.step_inst(&tank_status).unwrap();
    assert_eq!(tank_status.get_dir(), TankDirection::South);
    interprete.step_inst(&tank_status).unwrap();
    interprete.step_inst(&tank_status).unwrap();
}

#[test]
fn test_radar() {
    // (0,0)
    let mut status = TankStatus::default();
    assert_eq!(status.calc_radar(), 0);
    status.set_pos(3, 5);
    status.set_dir(TankDirection::West);
    assert_eq!(status.calc_radar(), 3);
    status.set_dir(TankDirection::North);
    assert_eq!(status.calc_radar(), 5);

    let mut interprete = Interpreter::new(
        "
                                          avanza;
                                          avanza;
                                          var x = radar;
                                          gira derecha;
                                          x = radar;
                                          gira derecha;
                                          x = radar;
                                          avanza;
                                          avanza;
                                          x = radar;
                                          ",
    )
    .unwrap();

    let status = interprete.step_inst(&status).unwrap();
    let status = interprete.step_inst(&status).unwrap();
    let status = interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x").unwrap(), 3);

    let status = interprete.step_inst(&status).unwrap();
    let status = interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x").unwrap(), 8);

    let status = interprete.step_inst(&status).unwrap();
    let status = interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x").unwrap(), 8);

    let status = interprete.step_inst(&status).unwrap();
    let status = interprete.step_inst(&status).unwrap();
    let status = interprete.step_inst(&status).unwrap();
    assert_eq!(interprete.get_var_value("x").unwrap(), 6);
}
