use std::collections::HashMap;

use crate::function::Function;
use crate::variable::Variable;

pub struct Scope {
    pub functions: HashMap<String, Function>,
    pub variables: HashMap<String, Variable>
}

impl Scope {
    pub fn new() -> Self {
        Self{functions: HashMap::new(), variables: HashMap::new()}
    }
}