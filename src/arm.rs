use crate::ast::AST;
use crate::emitter::{Emit, Environment};
use std::collections::HashMap;
use std::io::Write;

struct ArmCode {
    label_counter: u16,
}

impl Default for ArmCode {
    fn default() -> Self {
        ArmCode { label_counter: 0 }
    }
}

impl ArmCode {
    fn emit_fn_prologue(&mut self, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "\tpush {{fp, lr}}")?;
        writeln!(writer, "\tmov fp, sp")?;
        writeln!(writer, "\tpush {{r0, r1, r2, r3}}")
    }
    fn emit_fn_epilogue(&mut self, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "\tmov sp, fp")?;
        // .We set r0 and thus our return value to 0.
        // This is to mimic the fact that JavaScript functions return undefined
        // when there’s no explicit return
        writeln!(writer, "\tmov r0, #0")?;
        writeln!(writer, "\tpop {{ fp, pc }}")
    }
}

impl Emit for ArmCode {
    fn emit(&self) {
        todo!()
    }
    fn new_label(&mut self) -> String {
        self.label_counter = self.label_counter + 1;
        return format!(".L{}", self.label_counter);
    }

    fn write(
        &mut self,
        ast: &AST,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        match ast {
            AST::Main(statements) => self.emit_main(statements, env, writer),
            AST::Assert(condition) => self.emit_assert(condition, env, writer),
            AST::Number(number) => self.emit_number(number, env, writer),
            AST::Not(term) => self.emit_not(term, env, writer),
            AST::Add { left, right } => self.emit_add(left, right, env, writer),
            AST::Subtract { left, right } => self.emit_subtract(left, right, env, writer),
            AST::Multiply { left, right } => self.emit_multiply(left, right, env, writer),
            AST::Divide { left, right } => self.emit_divide(left, right, env, writer),
            AST::Equal { left, right } => self.emit_equal(left, right, env, writer),
            AST::NotEqual { left, right } => self.emit_not_equal(left, right, env, writer),
            AST::Block(statements) => self.emit_block(statements, env, writer),
            AST::Call { args, callee } => self.emit_call(args, callee, env, writer),
            AST::Function {
                name,
                parameters,
                body,
            } => self.emit_function(name, parameters, body, env, writer),
            AST::IfNode {
                conditional,
                consequence,
                alternative,
            } => self.emit_ifnode(conditional, consequence, alternative, env, writer),
            // AST::Id(_) => {}

            // AST::Return { .. } => {}

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
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, env, writer);
        write!(writer, "\t")?;
        writeln!(writer, "add r0, r0, r1")
    }

    fn emit_subtract(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, env, writer);
        write!(writer, "\t")?;
        writeln!(writer, "sub r0, r0, r1")
    }
    fn emit_multiply(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, env, writer);
        write!(writer, "\t")?;
        writeln!(writer, "mul r0, r0, r1")
    }
    fn emit_divide(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, env, writer);
        write!(writer, "\t")?;
        writeln!(writer, "udiv r0, r0, r1")
    }
    fn emit_equal(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, env, writer);
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
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, env, writer);
        writeln!(
            writer,
            r#"
    cmp r0, r1
    moveq r0, #0
    movne r0, #1
        "#
        )
    }
    fn emit_infix_operands(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) {
        self.write(left, env, writer).unwrap();
        writeln!(writer, "\tpush {{r0, ip}}").unwrap();
        self.write(right, env, writer).unwrap();
        writeln!(writer, "\tpop {{r1, ip}}").unwrap();
    }

    fn emit_not(
        &mut self,
        term: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.write(term, env, writer)?;
        writeln!(
            writer,
            r#"
    cmp r0, #0
    moveq r0, #1
    movne r0, #0    
        "#
        )
    }

    fn emit_number(
        &mut self,
        number: &u64,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        writeln!(writer, "\tldr r0, ={}", number)
    }

    ///
    ///
    fn emit_assert(
        &mut self,
        condition: &AST,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.write(condition, env, writer)?;
        writeln!(
            writer,
            r#"
    cmp r0, #1
    moveq r0, #'T'
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
    fn emit_main(
        &mut self,
        statements: &Vec<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        writeln!(writer, ".global main")?;
        writeln!(writer, "main:")?;
        writeln!(writer, "\tpush {{fp, lr}}")?;
        for statement in statements {
            self.write(statement, env, writer)?;
        }
        writeln!(writer, "\tmov r0, #0")?;
        writeln!(writer, "\tpop {{fp, pc}}")
    }

    fn emit_block(
        &mut self,
        statements: &Vec<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        for statement in statements {
            self.write(statement, env, writer)?;
        }
        Ok(())
    }

    fn emit_call(
        &mut self,
        args: &Vec<AST>,
        callee: &String,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        let len = args.len();
        if args.is_empty() {
            writeln!(writer, "\tbl {}", callee)
        } else if len == 1 {
            self.write(args.get(0).unwrap(), env, writer);
            writeln!(writer, "\tbl {}", callee)
        } else if len >= 2 && len <= 4 {
            // allocate enough stack space for up to four arguments (16 bytes)
            // We do that by subtracting from the stack
            // pointer since the stack grows from higher memory addresses to
            // lower.
            writeln!(writer, "\tsub sp, sp, #16")?;
            args.iter().enumerate().for_each(|(i, arg)| {
                self.write(arg, env, writer).expect("Write failed");
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

    fn emit_ifnode(
        &mut self,
        conditional: &Box<AST>,
        consequence: &Box<AST>,
        alternative: &Box<AST>,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        let if_false_label = self.new_label();
        let end_if_label = self.new_label();
        self.write(conditional, env, writer)?;
        writeln!(writer, "\tcmp r0, #0")?;
        writeln!(writer, "\tbeq {}", if_false_label)?;
        self.write(consequence, env, writer)?;
        writeln!(writer, "\tb {}", end_if_label)?;
        writeln!(writer, "{}:", if_false_label)?;
        self.write(alternative, env, writer)?;
        writeln!(writer, "{}:", end_if_label)
    }

    fn emit_function(
        &mut self,
        name: &String,
        parameters: &Vec<String>,
        body: &Box<AST>,
        _: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        if parameters.len() > 4 {
            panic!("More than 4 params is not supported");
        }
        writeln!(writer, "")?;
        writeln!(writer, ".global {}", name)?;
        writeln!(writer, "{}:", name)?;
        self.emit_fn_prologue(writer)?;
        let mut locals: HashMap<String, isize> = HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            locals.insert(parameter.clone(), 4 * i as isize - 16);
        }
        let env = Environment { locals: locals };
        self.write(body, Some(&env), writer)?;
        self.emit_fn_epilogue(writer)
    }

    fn emit_idnode(
        &mut self,
        name: &String,
        env: Option<&Environment>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        let offset = env
            .expect("Missing enviroment")
            .locals
            .get(name)
            .expect(format!("Undefined variable: {}", name).as_str());
        writeln!(writer, "\t ldr r0, [fp, #{}]", offset)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use std::fs::File;
    use std::process::{Command, Stdio};
    use std::{env, io};

    fn compile_and_run(code: &str) -> Result<()> {
        let ast = parse(code).expect("Failed");

        let mut arm_code = ArmCode {
            ..Default::default()
        };
        //arm_code.write(&ast, &mut io::stdout());

        arm_code.write(
            &ast,
            Option::None,
            &mut File::create("test.s").expect("Open file failed"),
        );

        // arm-linux-gnueabihf-gcc -static test.s

        let mut output = Command::new("arm-linux-gnueabihf-gcc")
            .arg("-static")
            .arg("test.s")
            .arg("-o")
            .arg("test.bin")
            .output()
            .expect("failed to execute process");

        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        assert!(output.status.success());

        output = Command::new("./test.bin")
            .output()
            .expect("failed to execute process");

        // io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        assert!(output.status.success())
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
                    putchar(46);
                    putchar(46);
                    var a = rand() != 42;
                }
            }"#,
        )
    }

    #[test]
    fn compile_if_1() {
        compile_and_run(
            r#"function main() {
                { 
                    if (1) {
                        assert(1);
                    } else {
                        assert(0);
                    }
                }
            }"#,
        )
    }
    #[test]
    fn compile_if_2() {
        compile_and_run(
            r#"function main() {
                { 
                    if (0) {
                        assert(0);
                    } else {
                        assert(1);
                    }
                }
            }"#,
        )
    }
    #[test]
    fn compile_function() {
        compile_and_run(
            r#"function main() {
                { 
                    function asserttt(a, b, c, d) {
                        assert(a == 1);
                        assert(b == 2);
                        assert(c == 3);
                        assert(d == 4);
                    }
                    
                    asserttt(1, 2, 5, 4, 5);
                    
                }
            }"#,
        )
    }
}
