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


#[cfg(test)]
mod tests {
    use crate::ast::AST;

    #[test]
    fn it_works() {
        AST::Equal { left: Box::new(AST::Id("x".to_string())), right: Box::new(AST::Id("y".to_string())) };
    }
}
