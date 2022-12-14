use std::fmt;

// pub trait AST {
//     fn equals(&self, other: &dyn AST) -> bool;
// }
//
//
// pub struct Node {
//     value: f64
// }
//
// impl AST for Node {
//     fn equals(&self, other: &dyn AST) -> bool {
//         o
//     }
// }
//
// pub struct Id {
//     value: f64
// }
//
// pub struct Not<'a> {
//     term: &'a dyn AST
// }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    Token(String),
    Number(u64),
    Id(String),
    Not(Box<AST>),
    Equal { left: Box<AST>, right: Box<AST> },
    NotEqual { left: Box<AST>, right: Box<AST> },
    Add { left: Box<AST>, right: Box<AST> },
    Subtract { left: Box<AST>, right: Box<AST> },
    Multiply { left: Box<AST>, right: Box<AST> },
    Divide { left: Box<AST>, right: Box<AST> },

    Call {callee: String, args: Vec<Box<AST>>},
    Return {term: Box<AST>},
    Block {statements: Vec<Box<AST>>},

    IfNode{conditional: Box<AST>, consequence: Box<AST>, alternative: Box<AST>},
    Function{name: String, parameters:Vec<String>, body: Box<AST>},
    Var{name: String,  value: Box<AST>},
    Assign{name: String,  value: Box<AST>},
    Wile{conditional: Box<AST>,  body: Box<AST>},
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            AST::Token(value) => write!(f, "TOKEN {}", value),
            AST::Number(value) => write!(f, "{}", value),
            AST::Id(name) => write!(f, "{}", name),
            AST::Not(value) => write!(f, "!{}", value),
            AST::Equal { left, right} => write!(f, "({} == {})",left, right),
            AST::NotEqual { left, right} => write!(f, "({} != {})",left, right),
            AST::Add { left, right} => write!(f, "({} + {})",left, right),
            AST::Subtract { left, right} => write!(f, "({} - {})",left, right),
            AST::Multiply  { left, right} => write!(f, "({} * {})",left, right),
            AST::Divide  { left, right} => write!(f, "({} / {})",left, right),
            AST::Call { callee, args } => write!(f, "{} ({:?})",callee, args),
            AST::Return { term} => write!(f, "return {} ",term),
            AST::Block { statements } => write!(f, "{{\n {:?} }}",statements),
            AST::IfNode { conditional, consequence, alternative } =>  write!(f, "if ({}) \n{{\n {:?} \n}} else {{ {:?}}} ",conditional, consequence, alternative),
            AST::Function { name, parameters, body } =>  write!(f, "function  {}({:?})  \n{{\n {:?} \n}}",name, parameters, body),
            AST::Var { name, value } => write!(f, "var {} = {}",name, value),
            AST::Assign  { name, value } => write!(f, " {} = {}",name, value),
            AST::Wile { conditional, body } => write!(f, "while ({}) \n{{\n {:?} \n}}  ",conditional, body),
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::ast::AST;

    #[test]
    fn it_works() {
        AST::Equal { left: Box::new(AST::Id("x".to_string())), right: Box::new(AST::Id("y".to_string())) };
    }

    #[test]
    fn add() {
        let ast = AST::Add {
            left: AST::Number(42).into(),
            right: AST::Not(
                AST::NotEqual {
                    left: AST::Number(20).into(),
                    right: AST::Number(20).into(),
                }.into()
            ).into()
        };

        println!("{}", ast);

    }
}
