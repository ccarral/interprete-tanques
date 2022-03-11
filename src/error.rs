#[derive(Debug)]
pub enum ErrorInterprete {
    VarNoDecl(String),
}

impl std::fmt::Display for ErrorInterprete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorInterprete::VarNoDecl(var) => {
                f.write_str(&format!("Variable no declarada previamente: {}", &var))
            }
        }
    }
}

impl std::error::Error for ErrorInterprete {}
