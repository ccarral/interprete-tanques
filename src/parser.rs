pub use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "gramatica.pest"]
pub struct ParserTanques;

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser;

    #[test]
    pub fn test_decl() {
        let decl_parse_result = ParserTanques::parse(Rule::decl, "var x=1;");
        assert!(decl_parse_result.is_ok());
        let decl_parse_result = ParserTanques::parse(Rule::decl, "var x = 1;");
        assert!(decl_parse_result.is_ok());
        let decl_parse_result = ParserTanques::parse(Rule::expr, "var x = 1;");
        assert!(decl_parse_result.is_ok());
    }

    #[test]
    pub fn test_prog() {
        let prog = ParserTanques::parse(Rule::prog, "var x = 1;\nvar wey = 4;");
        assert!(prog.is_ok());

        let prog = ParserTanques::parse(Rule::prog, "\nvar x = 1;\n\n");
        assert!(prog.is_ok());
    }
}
