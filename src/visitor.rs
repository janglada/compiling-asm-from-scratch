use crate::ast::AST;
use crate::visitor;
pub trait Visitor<T> {
    fn visit_assert(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_array_length(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_array_lookup(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_array_literal(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_boolean(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_number(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_id(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_not(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_equal(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_not_equal(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_add(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_subtract(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_multiply(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_divide(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_less_than(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_greater_than(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_less_than_equal(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_greater_than_equal(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_call(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_return(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_block(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_if(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_function(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_var(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_assign(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_while(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_undefined(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_null(&mut self, node: &AST) -> std::io::Result<T>;
    fn visit_main(&mut self, node: &AST) -> std::io::Result<T>;
}

pub trait AstVisitor {
    fn visit<T>(&self, v: &mut dyn Visitor<T>) -> std::io::Result<T>;
    fn equal(&self, node: &AST) -> bool;
}

impl AstVisitor for AST {
    fn visit<T>(&self, v: &mut dyn Visitor<T>) -> std::io::Result<T> {
        match self {
            AST::Number(_) => v.visit_number(self),
            AST::Id(_) => v.visit_id(self),
            AST::Not(_) => v.visit_not(self),
            AST::Equal { .. } => v.visit_equal(self),
            AST::NotEqual { .. } => v.visit_not_equal(self),
            AST::Add { .. } => v.visit_add(self),
            AST::Subtract { .. } => v.visit_subtract(self),
            AST::Multiply { .. } => v.visit_multiply(self),
            AST::Divide { .. } => v.visit_divide(self),
            AST::LessThan { .. } => v.visit_less_than(self),
            AST::GreaterThan { .. } => v.visit_greater_than(self),
            AST::LessThanEqual { .. } => v.visit_less_than_equal(self),
            AST::GreaterThanEqual { .. } => v.visit_greater_than_equal(self),
            AST::Call { .. } => v.visit_call(self),
            AST::Return { .. } => v.visit_return(self),
            AST::Block(_) => v.visit_block(self),
            AST::IfNode { .. } => v.visit_if(self),
            AST::Function { .. } => v.visit_function(self),
            AST::Var { .. } => v.visit_var(self),
            AST::Assign { .. } => v.visit_assign(self),
            AST::While { .. } => v.visit_while(self),
            AST::Undefined => v.visit_undefined(self),
            AST::Null => v.visit_null(self),
            AST::Boolean(_) => v.visit_boolean(self),
            AST::ArrayLiteral(_) => v.visit_array_literal(self),
            AST::ArrayLookup { .. } => v.visit_array_lookup(self),
            AST::ArrayLength(_) => v.visit_array_length(self),
            AST::Main(_) => v.visit_main(self),
            AST::Assert(_) => v.visit_assert(self),
        }
    }

    fn equal(&self, node: &AST) -> bool {
        // Implement equality comparison based on your AST definition
        std::mem::discriminant(self) == std::mem::discriminant(node)
    }
}
