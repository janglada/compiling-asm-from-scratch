use crate::ast::AST;
use crate::emitter::{Backend, Environment};
use std::collections::{HashMap, LinkedList};
use std::io::Write;

#[derive(Default)]
struct ArmBackend {
    label_counter: u16,
    env: LinkedList<Environment>,
}

impl ArmBackend {
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

impl Backend for ArmBackend {
    fn emit(&self) {
        todo!()
    }
    fn write(&mut self, ast: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
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
            AST::Function {
                name,
                parameters,
                body,
            } => self.emit_function(name, parameters, body, writer),
            AST::IfNode {
                conditional,
                consequence,
                alternative,
            } => self.emit_ifnode(conditional, consequence, alternative, writer),
            AST::Return { term } => self.emit_return(term, writer),
            AST::Id(name) => self.emit_idnode(name, writer),

            AST::Return { term } => self.emit_return(term, writer),

            // AST::Function { .. } => {}
            AST::Var { name, value } => self.emit_var(name, value, writer),
            AST::Assign { name, value } => self.emit_assign(name, value, writer),
            // AST::While { .. } => {}
            _ => {
                writeln!(writer)
            }
        }

        // if let AST::Main(statements) = ast {
        //     self.emit_main(statements, writer)
        // } else {
        //     // unreachable!()
        //     Ok(())
        // }
    }

    fn emit_not(&mut self, term: &Box<AST>, writer: &mut dyn Write) -> std::io::Result<()> {
        self.write(term, writer)?;
        writeln!(
            writer,
            r#" cmp r0, #0
    moveq r0, #1
    movne r0, #0"#
        )
    }
    fn emit_number(&mut self, number: &u64, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "\tldr r0, ={}", number)
    }

    ///
    ///
    fn emit_assert(&mut self, condition: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        self.write(condition, writer)?;
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r0, #1
    moveq r0, #'T'
    movne r0, #'F'
    bl putchar"#
        )
        // writeln!(writer, "\t moveq r0, #'.'");
        // writeln!(writer, "\t movne r0, #'F'");
        // writeln!(writer, "\t bl putchar");
    }
    ///
    ///
    ///
    fn emit_main(&mut self, statements: &Vec<AST>, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, ".global main")?;
        writeln!(writer, "main:")?;
        writeln!(writer, "\tpush {{fp, lr}}")?;
        for statement in statements {
            self.write(statement, writer)?;
        }
        writeln!(writer, "\tmov r0, #0")?;
        writeln!(writer, "\tpop {{fp, pc}}")
    }
    fn emit_block(&mut self, statements: &Vec<AST>, writer: &mut dyn Write) -> std::io::Result<()> {
        for statement in statements {
            self.write(statement, writer)?;
        }
        Ok(())
    }
    fn emit_add(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "\tadd r0, r1, r0")
    }
    fn emit_infix_operands(&mut self, left: &Box<AST>, right: &Box<AST>, writer: &mut dyn Write) {
        self.write(left, writer).unwrap();
        writeln!(writer, "\tpush {{r0, ip}}").unwrap();
        self.write(right, writer).unwrap();
        writeln!(writer, "\tpop {{r1, ip}}").unwrap();
    }
    fn emit_subtract(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "\tsub r0, r1, r0")
    }

    fn emit_divide(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "\tudiv r0, r0, r1")
    }

    fn emit_multiply(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        writeln!(writer, "\tmul r0, r0, r1")
    }

    fn emit_equal(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r0, r1
    moveq r0, #1
    movne r0, #0"#
        )
    }

    fn emit_not_equal(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.emit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r0, r1
    moveq r0, #0
    movne r0, #1"#
        )
    }

    fn emit_call(
        &mut self,
        args: &Vec<AST>,
        callee: &String,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        let len = args.len();
        if args.is_empty() {
            writeln!(writer, "\tbl {}", callee)
        } else if len == 1 {
            self.write(args.first().unwrap(), writer);
            writeln!(writer, "\tbl {}", callee)
        } else if (2..=4).contains(&len) {
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

    fn emit_function(
        &mut self,
        name: &String,
        parameters: &Vec<String>,
        body: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        if parameters.len() > 4 {
            panic!("More than 4 params is not supported");
        }
        writeln!(writer)?;
        writeln!(writer, ".global {}", name)?;
        writeln!(writer, "{}:", name)?;
        self.emit_fn_prologue(writer)?;

        let mut locals: HashMap<String, isize> = HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            locals.insert(parameter.clone(), 4 * i as isize - 16);
        }
        let env = Environment {
            locals,
            next_local_offset: -20,
        };
        self.env.push_back(env);
        self.write(body, writer)?;
        self.env.pop_back();
        self.emit_fn_epilogue(writer)
    }

    fn emit_ifnode(
        &mut self,
        conditional: &Box<AST>,
        consequence: &Box<AST>,
        alternative: &Box<AST>,

        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        let if_false_label = self.new_label();
        let end_if_label = self.new_label();
        self.write(conditional, writer)?;
        writeln!(writer, "\tcmp r0, #0")?;
        writeln!(writer, "\tbeq {}", if_false_label)?;
        self.write(consequence, writer)?;
        writeln!(writer, "\tb {}", end_if_label)?;
        writeln!(writer, "{}:", if_false_label)?;
        self.write(alternative, writer)?;
        writeln!(writer, "{}:", end_if_label)
    }

    fn emit_idnode(&mut self, name: &String, writer: &mut dyn Write) -> std::io::Result<()> {
        let env = self.env.iter().last().expect("Missing environment");
        let offset = env
            .locals
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {}", name));
        writeln!(writer, "\tldr r0, [fp, #{}]", offset)
    }

    fn emit_return(&mut self, term: &Box<AST>, writer: &mut dyn Write) -> std::io::Result<()> {
        self.write(term, writer)?;
        writeln!(writer, "\tmov sp, fp")?;
        writeln!(writer, "\tpop {{fp, pc}}")
    }
    fn emit_assign(
        &mut self,
        name: &String,
        value: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.write(value, writer)?;
        let env = self.env.iter().last().expect("Missing environment");
        let offset = env
            .locals
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {}", name));
        writeln!(writer, "\tstr r0, [fp, #{}]", offset)
    }
    fn emit_var(
        &mut self,
        name: &String,
        value: &Box<AST>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        self.write(value, writer)?;
        writeln!(writer, "\tpush {{r0, ip}}");
        let mut env = self.env.pop_back().expect("Missing environment");

        env.locals
            .insert(name.to_string(), env.next_local_offset - 4);
        env.next_local_offset -= 8;
        self.env.push_back(env);
        Ok(())
        // env.locals.set(this.name, env.nextLocalOffset - 4);
        // env.nextLocalOffset -= 8
    }

    fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!(".L{}", self.label_counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CompileError;
    use crate::parser::parse;
    use std::fs::File;
    use std::io;
    use std::process::Command;

    struct Output {
        pub stdout: Vec<u8>,
        pub stderr: Vec<u8>,
    }

    fn compile_and_run(code: &str) -> Result<Output, CompileError> {
        let mut env = LinkedList::new();
        env.push_back(Environment {
            locals: HashMap::new(),
            next_local_offset: 0,
        });

        let mut arm_code = ArmBackend {
            label_counter: 0,
            env,
        };
        //
        //     let file_base_name = format!("tmp/test_{:x}", md5::compute(code));
        //
        //     let ast = parse(code).expect("Parse error");
        //     let locals: HashMap<String, isize> = HashMap::new();
        //     println!("compiling....");
        //     arm_code
        //         .write(
        //             &ast,
        //             &mut File::create(format!("{}.s", file_base_name)).expect("Open file failed"),
        //         )
        //         .map_err(CompileError::IOError)
        //         .expect("Could not generate assembly");
        //
        //     //arm_code.write(&ast, &mut io::stdout());
        //     println!("assembly written");
        //     // arm-linux-gnueabihf-gcc -static test.s
        //
        //     let compile_result = Command::new("gcc")
        //         // .arg("-g")
        //         .arg(format!("{}.s", file_base_name))
        //         .arg("-o")
        //         .arg(format!("{}.bin", file_base_name))
        //         .output();
        //
        //     let _codegen_result: Result<(), CompileError> = match compile_result {
        //         Ok(output) => {
        //             io::stdout().write_all(&output.stdout).unwrap();
        //             io::stderr().write_all(&output.stderr).unwrap();
        //
        //             if output.status.success() {
        //                 println!("Compiled, executing...");
        //                 Ok(())
        //             } else {
        //                 println!("Compile error");
        //                 let errmsg = String::from_utf8_lossy(&output.stderr).into_owned();
        //                 Err(CompileError::CodeGenError(errmsg))
        //             }
        //         }
        //         Err(e) => Err(CompileError::CodeGenError(e.to_string())),
        //     };
        //
        //     _codegen_result.expect("Error");
        //     panic!("ERROR");
        //
        //     let execution_res = match Command::new(format!("./{}.bin", file_base_name)).output() {
        //         Ok(output) => {
        //             io::stdout().write_all(&output.stdout).unwrap();
        //             io::stderr().write_all(&output.stderr).unwrap();
        //
        //             if output.status.success() {
        //                 Ok(Output {
        //                     stdout: output.stdout.clone(),
        //                     stderr: output.stdout.clone(),
        //                 })
        //             } else {
        //                 let errmsg = String::from_utf8_lossy(&output.stderr).into_owned();
        //
        //                 return Err(CompileError::RuntimeError(errmsg, output.status.code()));
        //             }
        //         }
        //         Err(e) => return Err(CompileError::RuntimeError(e.to_string(), Option::None)),
        //     };
        //
        //     execution_res
        // }
        //
        // fn compile_and_run_x86(code: &str) -> Result<Output, CompileError> {
        //     let mut env = LinkedList::new();
        //     env.push_back(Environment {
        //         locals: HashMap::new(),
        //         next_local_offset: 0,
        //     });
        //
        //     let mut arm_code = ArmBackend {
        //         label_counter: 0,
        //         env,
        //     };

        let file_base_name = format!("tmp/test_{:x}", md5::compute(code));

        let ast = parse(code).expect("Parse error");
        let locals: HashMap<String, isize> = HashMap::new();

        arm_code
            .write(
                &ast,
                &mut File::create(format!("{}.s", file_base_name)).expect("Open file failed"),
            )
            .map_err(CompileError::IOError)
            .expect("Could not generate assembly");

        //arm_code.write(&ast, &mut io::stdout());
        println!("assembly written");
        // arm-linux-gnueabihf-gcc -static test.s

        let compile_result = Command::new("arm-linux-gnueabihf-gcc")
            // .arg("-g")
            .arg("-static")
            .arg(format!("{}.s", file_base_name))
            .arg("-o")
            .arg(format!("{}.bin", file_base_name))
            .output();

        let _codegen_result: Result<(), CompileError> = match compile_result {
            Ok(output) => {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();

                if output.status.success() {
                    println!("Compiled, executing...");
                    Ok(())
                } else {
                    println!("Compile error");
                    let errmsg = String::from_utf8_lossy(&output.stderr).into_owned();
                    Err(CompileError::CodeGenError(errmsg))
                }
            }
            Err(e) => Err(CompileError::CodeGenError(e.to_string())),
        };

        let execution_res = match Command::new(format!("./{}.bin", file_base_name)).output() {
            Ok(output) => {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();

                if output.status.success() {
                    Ok(Output {
                        stdout: output.stdout.clone(),
                        stderr: output.stdout.clone(),
                    })
                } else {
                    let errmsg = String::from_utf8_lossy(&output.stderr).into_owned();

                    Err(CompileError::RuntimeError(errmsg, output.status.code()))
                }
            }
            Err(e) => Err(CompileError::RuntimeError(e.to_string(), Option::None)),
        };

        execution_res
    }

    #[test]
    fn compile_main() {
        compile_and_run(
            r#"function main() {
                var b = 1;
                while (a) {
                    b = 1;
                }
            }"#,
        )
        .expect("TODO: panic message");
    }

    #[test]
    fn compile_assert() {
        let result = compile_and_run(
            r#"function main() {
                assert(0);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("F".to_string(), String::from_utf8(result.stdout).unwrap());

        let result = compile_and_run(
            r#"function main() {
                assert(1);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn compile_not() {
        compile_and_run(
            r#"function main() {
                assert(!1);
            }"#,
        );
    }

    #[test]
    fn compile_basic_infix() {
        let result = compile_and_run(
            r#"function main() {
                assert(42 == 42);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());

        let result_fail = compile_and_run(
            r#"function main() {
                assert(42 == 11111);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!(
            "F".to_string(),
            String::from_utf8(result_fail.stdout).unwrap()
        );
    }

    #[test]
    fn compile_infix() {
        let result = compile_and_run(
            r#"function main() {
                assert(42 == 4 + 2 * (12 - 2) + 3 * (5 + 1));
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn compile_infix2() {
        let result = compile_and_run(
            r#"function main() {
                assert(2==3-1);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn compile_infix3() {
        let result = compile_and_run(
            r#"function main() {
                assert(6 == 4 + (3-1) );
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
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
        .expect("Compile an run failed");
    }

    #[test]
    fn compile_call() {
        compile_and_run(
            r#"function main() {
                { 
                    putchar(46);
                    putchar(46);
                    putchar(46);

                }
            }"#,
        )
        .expect("Compile an run failed");
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
        .expect("Compile an run failed");
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
                     putchar(84);
                }
                
            }"#,
        )
        .expect("Compile an run failed");
    }

    #[test]
    fn compile_function_assert() {
        compile_and_run(
            r#"function main() 
                { 
                    assert1234(1,2,3,4);
                }
                function myassert(x) {
                    if (x) {
                        putchar(84);
                        putchar(10);
                    } else {
                        putchar(70);
                        putchar(10);
                    }
                   
                }
                function assert1234(a, b, c, d) {
                    putchar(a);
                    putchar(b);
                    putchar(c);
                    putchar(d);
                }                
            "#,
        )
        .expect("Compile an run failed");
    }

    #[test]
    fn compile_function() {
        compile_and_run(
            r#"function main() {                 
                    assert(1, 2, 3, 4);                    
                }
                
                function assert(a, b, c, d) {
                    assert(a == 1);
                    assert(b == 2);
                    assert(c == 3);
                    assert(d == 4);
                }                
                
            "#,
        )
        .expect("Compile an run failed");
    }

    #[test]
    fn main_return() {
        compile_and_run(
            r#"function main() {             
                return 0;
            }"#,
        )
        .expect("Compile an run failed");
    }

    #[test]
    fn factorial() {
        let result = compile_and_run(
            r#"function factorial(n) {
  if (n == 0) {
    return 1;
  } else {
    return n * factorial(n - 1);
  }
}

function main() {
   assert(6 == factorial(3));
  return 0;
}
            "#,
        )
        .expect("Compile and run failed");
        //println!("{}", String::from_utf8(result.stdout).unwrap());
        assert_eq!("T", String::from_utf8(result.stdout).unwrap());
    }
    #[test]
    fn empty_main() {
        compile_and_run(
            r#"function main() {

            }
            "#,
        )
        .expect("Compile an run failed");
    }
    #[test]
    fn create_var() {
        compile_and_run(
            r#"function main() {
                    var x = 1;
            }
            "#,
        )
        .expect("Compile an run failed");
    }
    #[test]
    fn local_var() {
        compile_and_run(
            r#"function main() {
                    var x = 1;
                    var y = 2;
                    var z = x + y;
            }
            "#,
        )
        .expect("Compile an run failed");
    }
    #[test]
    fn assign_local_var() {
        compile_and_run(
            r#"function main() {
                    var a = 1;
                    assert(a == 1);
                    a = 0;
                    assert(a == 0);
            }
            "#,
        )
        .expect("Compile an run failed");
    }
}
