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
            AST::Not(term) => self.emit_not(term, writter),
            AST::Add { left, right } => self.emit_add(left, right, writter),
            AST::Subtract { left, right } => self.emit_subtract(left, right, writter),
            AST::Multiply { left, right } => self.emit_multiply(left, right, writter),
            AST::Divide { left, right } => self.emit_divide(left, right, writter),
            AST::Equal { left, right } => self.emit_equal(left, right, writter),
            AST::NotEqual { left, right } => self.emit_not_equal(left, right, writter),
            // AST::Id(_) => {}
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
    fn emit_add(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "add r0, r0, r1")
    }

    fn emit_subtract(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "sub r0, r0, r1")
    }
    fn emit_multiply(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "mul r0, r0, r1")
    }
    fn emit_divide(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "udiv r0, r0, r1")
    }
    fn emit_equal(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(
            writer,
            r#"
            cmp r0, r1
            moveq r0, #1
            movne r0, #0
        "#
        )
    }
    fn emit_not_equal(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(
            writer,
            r#"
            cmp r0, r1
            moveq r0, #0
            movne r0, #1
        "#
        )
    }
    fn emit_infix_operands(&self, left: &Box<AST>, right: &Box<AST>, writer: &mut dyn Write) {
        self.write(left, writer).unwrap();
        writeln!(writer, "push {{r0, ip}}").unwrap();
        self.write(right, writer).unwrap();
        writeln!(writer, "pop {{r1, ip}}").unwrap();
    }

    fn emit_not(&self, term: &Box<AST>, writer: &mut dyn Write) -> std::io::Result<()> {
        self.write(term, writer)?;
        writeln!(
            writer,
            r#"
    cmp r0, #0
    moveq r0, #1
    movne r0, #0    
        "#
        )
    }

    fn emit_number(&self, number: &u64, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "\tldr r0, ={}", number)
    }

    ///
    ///
    fn emit_assert(&self, condition: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        self.write(condition, writer)?;
        writeln!(
            writer,
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
    use std::process::{Command, Stdio};
    use std::{env, io};

    fn compile_and_run(code: &str) {
        let ast = parse(code).expect("Failed");

        let arm_code = ArmCode {};
        //arm_code.write(&ast, &mut io::stdout());

        let mut buffer = File::create("hello.s").expect("Open file failed");
        arm_code.write(&ast, &mut buffer);

        let mut output = Command::new("bash")
            .arg("-c")
            // .env("PATH", "/usr/bin")
            .arg("ls -al /usr/bin/ar*")
            // .arg("whoami")
            // .arg("-static")
            // .arg("hello.s")
            // .arg("-o")
            // .arg("hello")
            // .args(["-static", "hello.s", "-o", "hello"])
            .output()
            .expect("failed to execute process");

        // let output = Command::new("ls")
        //     .output()
        //     .expect("ls command failed to start");

        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        assert!(output.status.success());
    }

    #[test]
    fn compile_main() {
        compile_and_run(
            r#"function main() {
                b = 1;
                while (a) {
                    b = 1;
                }
            }"#,
        );
    }

    #[test]
    fn compile_assert() {
        compile_and_run(
            r#"function main() {
                assert(0);
            }"#,
        )
    }
    #[test]
    fn compile_not() {
        compile_and_run(
            r#"function main() {
                assert(!1);
            }"#,
        )
    }

    #[test]
    fn compile_infix() {
        compile_and_run(
            r#"function main() {
                assert(42 == 4 + 2 * (12 - 2) + 3 * (5 + 1));
            }"#,
        )
    }
}
