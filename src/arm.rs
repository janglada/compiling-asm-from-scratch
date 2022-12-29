use crate::ast::{Emit, AST};
use std::io::Write;

struct ArmCode {}

impl Emit for ArmCode {
    fn emit(&self) {
        todo!()
    }

    fn write(&self, ast: &AST, writter: &mut dyn Write) -> std::io::Result<()> {
        match ast {
            AST::Main(statements) => self.emit_main(statements, writter),
            AST::Assert(condition) => self.emit_assert(condition, writter),
            AST::Number(number) => self.emit_number(number, writter),
            // AST::Id(_) => {}
            // AST::Not(_) => {}
            // AST::Equal { .. } => {}
            // AST::NotEqual { .. } => {}
            // AST::Add { .. } => {}
            // AST::Subtract { .. } => {}
            // AST::Multiply { .. } => {}
            // AST::Divide { .. } => {}
            // AST::Call { .. } => {}
            // AST::Return { .. } => {}
            // AST::Block(_) => {}
            // AST::IfNode { .. } => {}
            // AST::Function { .. } => {}
            // AST::Var { .. } => {}
            // AST::Assign { .. } => {}
            // AST::While { .. } => {}
            _ => {
                writeln!(writter, "")
            }
        }

        // if let AST::Main(statements) = ast {
        //     self.emit_main(statements, writter)
        // } else {
        //     // unreachable!()
        //     Ok(())
        // }
    }

    fn emit_number(&self, number: &u64, writter: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writter, "\tldr r0, ={}", number)
    }

    ///
    ///
    fn emit_assert(&self, condition: &AST, writter: &mut dyn Write) -> std::io::Result<()> {
        self.write(condition, writter)?;
        writeln!(
            writter,
            r#"
    cmp r0, #1
    moveq r0, #'.'
    movne r0, #'F'
    bl putchar        
        "#
        )
        // writeln!(writter, "\t moveq r0, #'.'");
        // writeln!(writter, "\t movne r0, #'F'");
        // writeln!(writter, "\t bl putchar");
    }

    ///
    ///
    ///
    fn emit_main(&self, statements: &Vec<AST>, writter: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writter, ".global main")?;
        writeln!(writter, "main:")?;
        writeln!(writter, "\tpush {{fp, lr}}")?;
        for statement in statements {
            self.write(statement, writter)?;
        }
        writeln!(writter, "\tmov r0, #0")?;
        writeln!(writter, "\tpop {{fp, pc}}")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use std::fs::File;
    use std::io;

    #[test]
    fn compile_main() {
        let ast = parse(
            r#"function main() {
                b = 1;
                while (a) {
                    b = 1;
                }
            }"#,
        )
        .expect("Failed");

        let arm_code = ArmCode {};
        arm_code.write(&ast, &mut io::stdout());

        let mut buffer = File::create("compile_main.s").expect("Open file failed");
        arm_code.write(&ast, &mut buffer);
        // arm-linux-gnueabihf-gcc -static hello.s -o hello
    }

    #[test]
    fn compile_assert() {
        let ast = parse(
            r#"function main() {
                assert(0);
            }"#,
        )
        .expect("Failed");

        let arm_code = ArmCode {};
        arm_code.write(&ast, &mut io::stdout());

        let mut buffer = File::create("hello.s").expect("Open file failed");
        arm_code.write(&ast, &mut buffer);
        // arm-linux-gnueabihf-gcc -static hello.s -o hello
    }
}
