use crate::ast::AST;
use std::io::Write;

pub trait Emit {
    fn emit(&self);
    fn write(&mut self, ast: &AST, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_not(&mut self, term: &Box<AST>, writer: &mut dyn Write) -> std::io::Result<()>;
    fn emit_number(&mut self, number: &u64, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_assert(&mut self, condition: &AST, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_main(&mut self, statements: &Vec<AST>, writter: &mut dyn Write) -> std::io::Result<()>;
    fn emit_block(&mut self, statements: &Vec<AST>, writter: &mut dyn Write)
        -> std::io::Result<()>;
    fn emit_add(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_infix_operands(&mut self, left: &Box<AST>, right: &Box<AST>, writer: &mut dyn Write);
    fn emit_subtract(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_divide(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_multiply(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_equal(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
    fn emit_not_equal(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;

    fn emit_call(
        &mut self,
        args: &Vec<AST>,
        callee: &String,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;

    fn emit_ifnode(
        &mut self,
        conditional: &Box<AST>,
        consequence: &Box<AST>,
        alternative: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;

    fn new_label(&mut self) -> String;
}
