use crate::ast::AST;
use crate::emitter::Emit;
use std::io::Write;

struct ArmCode {}

impl Emit for ArmCode {
    fn emit(&self) {
        todo!()
    }

    fn write(&self, ast: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        match ast {
            AST::Main(statements) => self.emit_main(statements, writer),
            AST::Assert(condition) => self.emit_assert(condition, writer),
            AST::Number(number) => self.emit_number(number, writer),
            AST::Not(term) => self.emit_not(term, writer),
            AST::Add { left, right } => self.emit_add(left, right, writer),
            AST::Subtract { left, right } => self.emit_subtract(left, right, writer),
            AST::Multiply { left, right } => self.emit_multiply(left, right, writer),
            AST::Divide { left, right } => self.emit_divide(left, right, writer),
            AST::Equal { left, right } => self.emit_equal(left, right, writer),
            AST::NotEqual { left, right } => self.emit_not_equal(left, right, writer),
            AST::Block(statements) => self.emit_block(statements, writer),
            AST::Call { args, callee } => self.emit_call(args, callee, writer),
            // AST::Id(_) => {}

            // AST::Return { .. } => {}

            // AST::IfNode { .. } => {}
            // AST::Function { .. } => {}
            // AST::Var { .. } => {}
            // AST::Assign { .. } => {}
            // AST::While { .. } => {}
            _ => {
                writeln!(writer, "")
            }
        }

        // if let AST::Main(statements) = ast {
        //     self.emit_main(statements, writer)
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
        write!(writer, "\t")?;
        writeln!(writer, "add r0, r0, r1")
    }

    fn emit_subtract(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(writer, "sub r0, r0, r1")
    }
    fn emit_multiply(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(writer, "mul r0, r0, r1")
    }
    fn emit_divide(
        &self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
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
        writeln!(writer, "\tpush {{r0, ip}}").unwrap();
        self.write(right, writer).unwrap();
        writeln!(writer, "\tpop {{r1, ip}}").unwrap();
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
        // writeln!(writer, "\t moveq r0, #'.'");
        // writeln!(writer, "\t movne r0, #'F'");
        // writeln!(writer, "\t bl putchar");
    }

    ///
    ///
    ///
    fn emit_main(&self, statements: &Vec<AST>, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, ".global main")?;
        writeln!(writer, "main:")?;
        writeln!(writer, "\tpush {{fp, lr}}")?;
        for statement in statements {
            self.write(statement, writer)?;
        }
        writeln!(writer, "\tmov r0, #0")?;
        writeln!(writer, "\tpop {{fp, pc}}")
    }

    fn emit_block(&self, statements: &Vec<AST>, writer: &mut dyn Write) -> std::io::Result<()> {
        for statement in statements {
            self.write(statement, writer)?;
        }
        Ok(())
    }

    fn emit_call(
        &self,
        args: &Vec<AST>,
        callee: &String,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        let len = args.len();
        if args.is_empty() {
            writeln!(writer, "\tbl {}", callee)
        } else if len == 1 {
            self.write(args.get(0).unwrap(), writer);
            writeln!(writer, "\tbl {}", callee)
        } else if len >= 2 && len <= 4 {
            // allocate enough stack space for up to four arguments (16 bytes)
            // We do that by subtracting from the stack
            // pointer since the stack grows from higher memory addresses to
            // lower.
            writeln!(writer, "\tsub sp, sp, #16")?;
            args.iter().enumerate().for_each(|(i, arg)| {
                self.write(arg, writer).expect("Write failed");
                // We multiply by four to convert array indexes 0, 1, 2, 3 into
                // stack offsets in bytes: 0, 4, 8, 12.
                writeln!(writer, "\tstr r0, [sp, #{}]", 4 * i).expect("Write failed");
            });
            writeln!(writer, "\tpop {{r0, r1, r2, r3}}")?;
            writeln!(writer, "\tbl {}", callee)
        } else {
            panic!("More than 4 arguments are not supported in function calls")
        }
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

        let mut buffer = File::create("test.s").expect("Open file failed");
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
    #[test]
    fn compile_block() {
        compile_and_run(
            r#"function main() {
                { 
                    assert(1);
                    assert(1);
                }
            }"#,
        )
    }
    #[test]
    fn compile_call() {
        compile_and_run(
            r#"function main() {
                { 
                    putchar(46);
                    var a = rand() != 42;
                }
            }"#,
        )
    }
}
