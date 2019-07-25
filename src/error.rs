use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Debug)]
pub enum RuntimeError {
    DivByZero,
    NaNFloatResult,
    AnsNotDefined,
    VarNotDefined(String)
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::RuntimeError::*;
        match self {
            DivByZero           => write!(f, "cannot divide by zero."),
            NaNFloatResult      => write!(f, "floating point calculation didn't return a valid value. Maybe you tried to divide by zero?"),
            AnsNotDefined       => write!(f, "ans is not defined. Run at least one expression before using this variable."),
            VarNotDefined(v)    => write!(f, "variable {var} is not defined. Set it with {var} = <expression>.", var = v)
        }
    }
}