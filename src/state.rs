use super::number::Number;

use std::collections::HashMap;

pub struct State {
    ans: Option<Number>,
    vars: HashMap<String, Number>
}

impl State {
    pub fn new() -> Self {
        State {
            ans: None,
            vars: HashMap::new()
        }
    }

    pub fn set_ans(&mut self, new_val: Number) {
        self.ans = Some(new_val);
    }

    pub fn get_ans(&self) -> Option<Number> {
        self.ans.clone()
    }

    pub fn set_var(&mut self, var: &str, new_val: Number) {
        self.vars.insert(var.to_string(), new_val);
    }

    pub fn get_var(&mut self, var: &str) -> Option<Number> {
        self.vars.get(&var.to_string()).cloned()
    }
}