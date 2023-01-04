use crate::ast::AST;
use std::io::Write;

pub trait Emit {
    fn emit(&self);
    fn write(&self, ast: &AST, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_not(&self, term: &Box<AST>, writer: &mut dyn Write) -> std::io::Result<()>;
    fn emit_number(&self, number: &u64, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_assert(&self, condition: &AST, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_main(&self, statements: &Vec<AST>, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_block(&self, statements: &Vec<AST>, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_add(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_infix_operands(&self, left: &Box<AST>, right: &Box<AST>, writer: &mut dyn Write);
    fn emit_subtract(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_divide(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_multiply(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_equal(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_not_equal(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;

    fn emit_call(
        &self,
        args: &Vec<AST>,
        callee: &String,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
}
