use crate::ast::AST;
pub trait Visitor<T, W> {
    fn visit_assert(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_print(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_array_length(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_array_lookup(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_array_literal(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_boolean(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_number(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_id(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_not(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_equal(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_not_equal(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_add(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_subtract(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_multiply(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_divide(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_less_than(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_greater_than(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_less_than_equal(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_greater_than_equal(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_call(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_return(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_block(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_if(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_function(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_var(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_assign(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_while(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_undefined(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_null(&mut self, node: &AST, w: W) -> std::io::Result<T>;
    fn visit_main(&mut self, node: &AST, w: W) -> std::io::Result<T>;
}

pub trait AstVisitor {
    fn visit<T, W>(&self, v: &mut dyn Visitor<T, W>, w: W) -> std::io::Result<T>;
    fn equal(&self, node: &AST) -> bool;
}

impl AstVisitor for AST {
    fn visit<T, W>(&self, v: &mut dyn Visitor<T, W>, w: W) -> std::io::Result<T> {
        match self {
            AST::Number(_) => v.visit_number(self, w),
            AST::Id(_) => v.visit_id(self, w),
            AST::Not(_) => v.visit_not(self, w),
            AST::Equal { .. } => v.visit_equal(self, w),
            AST::NotEqual { .. } => v.visit_not_equal(self, w),
            AST::Add { .. } => v.visit_add(self, w),
            AST::Subtract { .. } => v.visit_subtract(self, w),
            AST::Multiply { .. } => v.visit_multiply(self, w),
            AST::Divide { .. } => v.visit_divide(self, w),
            AST::LessThan { .. } => v.visit_less_than(self, w),
            AST::GreaterThan { .. } => v.visit_greater_than(self, w),
            AST::LessThanEqual { .. } => v.visit_less_than_equal(self, w),
            AST::GreaterThanEqual { .. } => v.visit_greater_than_equal(self, w),
            AST::Call { .. } => v.visit_call(self, w),
            AST::Return { .. } => v.visit_return(self, w),
            AST::Block(_) => v.visit_block(self, w),
            AST::IfNode { .. } => v.visit_if(self, w),
            AST::Function { .. } => v.visit_function(self, w),
            AST::Var { .. } => v.visit_var(self, w),
            AST::Assign { .. } => v.visit_assign(self, w),
            AST::While { .. } => v.visit_while(self, w),
            AST::Undefined => v.visit_undefined(self, w),
            AST::Null => v.visit_null(self, w),
            AST::Boolean(_) => v.visit_boolean(self, w),
            AST::ArrayLiteral(_) => v.visit_array_literal(self, w),
            AST::ArrayLookup { .. } => v.visit_array_lookup(self, w),
            AST::ArrayLength(_) => v.visit_array_length(self, w),
            AST::Main(_) => v.visit_main(self, w),
            AST::Assert(_) => v.visit_assert(self, w),
            AST::Print(_) => v.visit_print(self, w),
        }
    }

    fn equal(&self, node: &AST) -> bool {
        // Implement equality comparison based on your AST definition
        std::mem::discriminant(self) == std::mem::discriminant(node)
    }
}
