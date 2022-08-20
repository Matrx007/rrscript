use crate::{string_eater::StringEater, scope::Scope, parse_error::ParseError};

/* fn parse_multiple_options(eater: StringEater, scope: &mut Scope, options: &[&dyn Fn(StringEater, &mut Scope) -> Result<(), ParseError>]) -> Result<(), ParseError> {
    
    for option in options {
        option()
    }
} */

pub fn parse_scope(eater: StringEater, scope: &mut Scope) -> Result<(), ParseError>{
    let mut token = eater.begin_token();



    eater.end_token(&mut token);

    Ok(())
}

pub fn parse(eater: StringEater) -> Scope {
    let mut scope = Scope::new();

    

    scope
}