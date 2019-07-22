use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Debug)]
pub enum RuntimeError {
    AnsNotDefined,
    VarNotDefined(String)
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::RuntimeError::*;
        match self {
            AnsNotDefined       => write!(f, "{}", "ans is not defined. Run at least one expression before using this variable."),
            VarNotDefined(v)    => write!(f, "{}", format!("variable {var} is not defined. Set it with {var} = <expression>.", var = v).as_str())
        }
    }
}