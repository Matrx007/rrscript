use crate::{scope::Scope, string_eater::StringEater, parser::parse};

pub struct Runner {
    pub root_scope: Scope,
    pub strings: Vec<String>
}

impl Runner {
    pub fn new() -> Self { Self { root_scope: Scope::new(), strings: Vec::new() } }

    pub fn source(&mut self, str: String) {
        let mut eater = StringEater::new(&str);

        let scope = parse(eater);

        self.strings.push(str);
    }
}