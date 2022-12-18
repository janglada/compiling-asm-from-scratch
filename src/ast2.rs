

pub trait ASTStruct {}

#[derive(PartialEq, PartialOrd)]
pub struct Number {
    value: f64,
}

impl Number {

}

#[derive(PartialEq, PartialOrd)]
pub struct Id {
    value: f64,
}

pub struct Not {
    term: Box<dyn ASTStruct>,
}

//
// pub struct Equal {
//     left: Box<dyn AST>,
//     right: Box<dyn AST>,
// }
//
// pub struct NotEqual {
//     left: Box<dyn AST>,
//     right: Box<dyn AST>,
// }
//
// pub struct Add {
//     left: Box<dyn AST>,
//     right: Box<dyn AST>,
// }
//
// pub struct Subtract {
//     left: Box<dyn AST>,
//     right: Box<dyn AST>,
// }
//
// pub struct Multiply {
//     left: Box<dyn AST>,
//     right: Box<dyn AST>,
// }
//
// pub struct Divide {
//     left: Box<dyn AST>,
//     right: Box<dyn AST>,
// }
//
// pub struct Call {
//     callee: String,
//     args: Vec<Box<dyn AST>>,
// }
//
// pub struct Return {
//     term: Box<dyn AST>,
// }
//
// pub struct Block {
//     statements: Vec<Box<dyn AST>>,
// }
//
// pub struct IfNode {
//     conditional: Box<dyn AST>,
//     consequence: Box<dyn AST>,
//     alternative: Box<dyn AST>,
// }
//
// pub struct Function {
//     name: String,
//     parameters: Vec<String>,
//     body: Box<dyn AST>,
// }
//
// pub struct Var {
//     name: String,
//     value: Box<dyn AST>,
// }
//
// pub struct Assign {
//     name: String,
//     value: Box<dyn AST>,
// }
//
// pub struct While {
//     conditional: Box<dyn AST>,
//     body: Box<dyn AST>,
// }

impl ASTStruct for Number {}
impl ASTStruct for Not {}
// impl AST for Equal {}
// impl AST for NotEqual {}
// impl AST for Add {}
// impl AST for Subtract {}
// impl AST for Multiply {}
// impl AST for Divide {}
// impl AST for Call {}
// impl AST for Return {}
// impl AST for Block {}
// impl AST for IfNode {}
// impl AST for Function {}
// impl AST for Var {}
// impl AST for Assign {}
// impl AST for While {}
