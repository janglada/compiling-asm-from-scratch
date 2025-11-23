use crate::ast::AST;
use crate::emitter::Environment;
use crate::visitor::{AstVisitor, Visitor};
use std::collections::{HashMap, LinkedList};
use std::io::Write;

struct ArmCodeGenerator {
    pub(crate) locals: HashMap<String, isize>,
    pub(crate) next_local_offset: isize,
    label_counter: i16,
}

impl Default for ArmCodeGenerator {
    fn default() -> ArmCodeGenerator {
        ArmCodeGenerator {
            locals: Default::default(),
            next_local_offset: 0,
            label_counter: 0,
        }
    }
}

impl ArmCodeGenerator {
    fn visit_infix_operands(
        &mut self,
        left: &Box<AST>,
        right: &Box<AST>,
        writer: &mut dyn Write,
    ) -> () {
        left.visit(self, writer);
        writeln!(writer, "\tpush {{r0, ip}}");
        right.visit(self, writer);
        writeln!(writer, "\tpop {{r1, ip}}");
    }
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
    fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!(".L{}", self.label_counter)
    }
}
impl Visitor<(), &mut dyn Write> for ArmCodeGenerator {
    fn visit_assert(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Assert(condition) = node else {
            panic!("Expected Assert node, got: {:?}", node)
        };
        condition.visit(self, writer)?;
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r0, #1
            moveq r0, #'T'
            movne r0, #'F'
            bl putchar"#
        )
    }

    fn visit_array_length(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::ArrayLength(array) = node else {
            panic!("Expected Assert node, got: {:?}", node)
        };
        array.visit(self, writer)?;
        writeln!(writer, "\tldr r0, [r0, #0]")
    }

    fn visit_array_lookup(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::ArrayLookup { array, index } = node else {
            panic!("Expected ArrayLookup node, got: {:?}", node)
        };
        array.visit(self, writer)?;
        writeln!(writer, "\tpush {{r0, ip}}");
        index.visit(self, writer)?;
        writeln!(writer, "\tpop {{r1, ip}}");
        writeln!(writer, "\tldr r2, [r1]");
        writeln!(writer, "\tcmp r0, r2");
        writeln!(writer, "\tmovhs r0, #0");
        writeln!(writer, "\taddlo r1, r1, #4");
        writeln!(writer, "\tlsllo r0, r0, #2");
        writeln!(writer, "\tldrlo r0, [r1, r0]")
    }

    fn visit_array_literal(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::ArrayLiteral(array_items) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };
        let len = array_items.len();
        writeln!(writer, "\tldr r0, ={}", 4 * (len + 1));
        writeln!(writer, "\tbl malloc");
        writeln!(writer, "\tpush {{r4, ip}}");
        writeln!(writer, "\tmov r4, r0");
        writeln!(writer, "\tldr r0, ={}", len);
        writeln!(writer, "\tstr r0, [r4]");
        for (i, item) in array_items.iter().enumerate() {
            item.visit(self, writer)?;
            writeln!(writer, "\tstr r0, [r4, #{}]", 4 * (i + 1));
        }

        writeln!(writer, "\tmov r0, r4");
        writeln!(writer, "\tpop {{r4, ip}}")
    }

    fn visit_boolean(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Boolean(value) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };
        writeln!(writer, "\tmov r0, #{}", if *value { 1 } else { 0 })
    }

    fn visit_number(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Number(number) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };
        writeln!(writer, "\tldr r0, ={}", *number)
    }

    fn visit_id(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Id(name) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };

        let offset = self
            .locals
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {}", name));
        writeln!(writer, "\tldr r0, [fp, #{}]", offset)
    }

    fn visit_not(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Not(term) = node else {
            panic!("Expected Not node, got: {:?}", node)
        };
        term.visit(self, writer)?;
        writeln!(
            writer,
            r#" cmp r0, #0
    moveq r0, #1
    movne r0, #0"#
        )
    }

    fn visit_equal(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Equal { left, right } = node else {
            panic!("Expected Not node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r0, r1
    moveq r0, #1
    movne r0, #0"#
        )
    }

    fn visit_not_equal(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::NotEqual { left, right } = node else {
            panic!("Expected NotEqual node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r0, r1
    moveq r0, #0
    movne r0, #1"#
        )
    }

    fn visit_add(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Add { left, right } = node else {
            panic!("Expected NotEqual node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(writer, "add r0, r1, r0")
    }

    fn visit_subtract(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Subtract { left, right } = node else {
            panic!("Expected NotEqual node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(writer, "sub r0, r1, r0")
    }

    fn visit_multiply(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Multiply { left, right } = node else {
            panic!("Expected Multiply node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(writer, "sub r0, r0, r1")
    }

    fn visit_divide(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Divide { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(writer, "udiv r0, r1, r0")
    }

    fn visit_less_than(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::LessThan { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r1, r0
    movlt r0, #1
    movge r0, #0"#
        )
    }

    fn visit_greater_than(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::GreaterThan { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r1, r0
    movgt r0, #1
    movle r0, #0"#
        )
    }

    fn visit_less_than_equal(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::LessThanEqual { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r1, r0
    movle r0, #1
    movgt r0, #0"#
        )
    }

    fn visit_greater_than_equal(
        &mut self,
        node: &AST,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        let AST::GreaterThanEqual { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right, writer);
        write!(writer, "\t")?;
        writeln!(
            writer,
            r#"cmp r1, r0
    movge r0, #1
    movlt r0, #0"#
        )
    }

    fn visit_call(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Call { args, callee } = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        let len = args.len();
        if args.is_empty() {
            writeln!(writer, "\tbl {}", callee)
        } else if len == 1 {
            args.first().unwrap().visit(self, writer)?;
            writeln!(writer, "\tbl {}", callee)
        } else if (2..=4).contains(&len) {
            // allocate enough stack space for up to four arguments (16 bytes)
            // We do that by subtracting from the stack
            // pointer since the stack grows from higher memory addresses to
            // lower.
            writeln!(writer, "\tsub sp, sp, #16")?;
            args.iter().enumerate().for_each(|(i, arg)| {
                arg.visit(self, writer);
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

    fn visit_return(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Return { term } = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        term.visit(self, writer)?;
        writeln!(writer, "\tmov sp, fp")?;
        writeln!(writer, "\tpop {{fp, pc}}")
    }

    fn visit_block(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Block(statements) = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        for statement in statements {
            statement.visit(self, writer)?;
        }
        Ok(())
    }

    fn visit_if(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::IfNode {
            conditional,
            consequence,
            alternative,
        } = node
        else {
            panic!("Expected Call node, got: {:?}", node)
        };
        let if_false_label = self.new_label();
        let end_if_label = self.new_label();
        conditional.visit(self, writer)?;
        writeln!(writer, "\tcmp r0, #0")?;
        consequence.visit(self, writer)?;
        writeln!(writer, "\tb {}", end_if_label)?;
        writeln!(writer, "{}:", if_false_label)?;
        alternative.visit(self, writer)?;
        writeln!(writer, "{}:", end_if_label)
    }

    fn visit_function(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Function {
            name,
            parameters,
            body,
        } = node
        else {
            panic!("Expected Call node, got: {:?}", node)
        };
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
        // let env = Environment {
        //     locals,
        //     next_local_offset: -20,
        // };
        let mut code_gen_visitor = ArmCodeGenerator {
            label_counter: 0,
            locals,
            next_local_offset: -20,
        };
        body.visit(&mut code_gen_visitor, writer)?;
        // self.env.push_back(env);
        // self.write(body, writer)?;
        // self.env.pop_back();
        self.emit_fn_epilogue(writer);
        Ok(())
    }

    fn visit_var(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Var { name, value } = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        value.visit(self, writer)?;
        writeln!(writer, "\tpush {{r0, ip}}");
        self.locals
            .insert(name.to_string(), self.next_local_offset - 4);
        self.next_local_offset -= 8;
        Ok(())
    }

    fn visit_assign(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Assign { name, value } = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        value.visit(self, writer)?;
        let offset = self
            .locals
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {}", name));
        writeln!(writer, "\tstr r0, [fp, #{}]", offset)
    }

    fn visit_while(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::While { conditional, body } = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        let loop_start = self.new_label();
        let loop_end = self.new_label();
        writeln!(writer, "{}:", loop_start)?;
        conditional.visit(self, writer)?;
        writeln!(writer, "\tcmp r0, #0")?;
        writeln!(writer, "\tbeq {}", loop_end)?;
        body.visit(self, writer)?;
        writeln!(writer, "\tb {}", loop_start)?;
        writeln!(writer, "{}:", loop_end)
    }

    fn visit_undefined(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "\tmov r0, #0")
    }

    fn visit_null(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "\tmov r0, #0")
    }

    fn visit_main(&mut self, node: &AST, writer: &mut dyn Write) -> std::io::Result<()> {
        let AST::Main(statements) = node else {
            panic!("Expected Main, got: {:?}", node)
        };
        writeln!(writer, ".global main")?;
        writeln!(writer, "main:")?;
        writeln!(writer, "\tpush {{fp, lr}}")?;
        for statement in statements {
            statement.visit(self, writer)?;
        }
        writeln!(writer, "\tmov r0, #0")?;
        writeln!(writer, "\tpop {{fp, pc}}")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CompileError;
    use crate::parser::parse;
    use rand::distributions::Alphanumeric;
    use rand::Rng;
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
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let file_base_name = format!("tmp/test_{}", s.to_string());

        let mut generator: ArmCodeGenerator = Default::default();
        let ast = parse(code).expect("Parse error");
        ast.visit(
            &mut generator,
            &mut File::create(format!("{}.s", file_base_name)).expect("Open file failed"),
        );

        println!("assembly written");
        let compile_result = Command::new("arm-linux-gnueabihf-gcc")
            // .arg("-g")
            .arg("-march=armv8-a")
            // .arg("-mcpu=cortex-m3")
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
    fn while_ne() {
        let result = compile_and_run(
            r#"function main() {
                var b = 1;
                while (b != 10) {
                    b = b + 1;
                }
                assert(b == 10);
            }"#,
        )
        .expect("TODO: panic message");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn while_lt() {
        let result = compile_and_run(
            r#"function main() {
                var b = 0;
                while (b <= 10000) {
                    b = b + 1;
                }
                assert(b == 10001);
            }"#,
        )
        .expect("TODO: panic message");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn while_cond() {
        let result = compile_and_run(
            r#"function main() {
                    var a = 1;
                    while(a != 100) {
                        a = a+1;
                    }
                    assert(a == 100);
                }"#,
        )
        .expect("TODO: panic message");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn assert() {
        let result = compile_and_run(
            r#"function main() {
                assert(0);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("F".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn assert_boolean() {
        let result = compile_and_run(
            r#"function main() {
                assert(true);
                assert(false);
            }"#,
        )
        .expect("Compile an run failed");
        assert_eq!("TF".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn assert_null() {
        let result = compile_and_run(
            r#"function main() {
                assert(null);
            }"#,
        )
        .expect("Compile an run failed");
        assert_eq!("F".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn assert_undefined() {
        let result = compile_and_run(
            r#"function main() {
                assert(undefined);
            }"#,
        )
        .expect("Compile an run failed");
        assert_eq!("F".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn not() {
        compile_and_run(
            r#"function main() {
                assert(!1);
            }"#,
        );
    }

    #[test]
    fn basic_infix() {
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
    fn infix() {
        let result = compile_and_run(
            r#"function main() {
                assert(42 == 4 + 2 * (12 - 2) + 3 * (5 + 1));
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn infix2() {
        let result = compile_and_run(
            r#"function main() {
                assert(2==3-1);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn infix3() {
        let result = compile_and_run(
            r#"function main() {
                assert(6 == 4 + (3-1) );
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn comparison() {
        let result = compile_and_run(
            r#"function main() {
                assert(4 < 5);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }
    #[test]
    fn comparison_gt() {
        let result = compile_and_run(
            r#"function main() {
                assert(8 > 7);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }
    #[test]
    fn comparison_ge() {
        let result = compile_and_run(
            r#"function main() {
                assert(2 >= 1);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn comparison3() {
        let result = compile_and_run(
            r#"function main() {            
                assert(34 <= 102);
                assert(1 <= 2);
                assert(1 > 2);
                assert(1 >= 2);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!(
            "TTFF".to_string(),
            String::from_utf8(result.stdout).unwrap()
        );
    }
    #[test]
    fn comparison6() {
        let result = compile_and_run(
            r#"function main() {
                var i = 0;
                assert(i == 0);
                i =  i + 1;
                assert(i == 0);
                assert(i == 1);
                i =  i + 1;
                assert(i == 2);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!(
            "TFTT".to_string(),
            String::from_utf8(result.stdout).unwrap()
        );
    }

    #[test]
    fn comparison4() {
        let result = compile_and_run(
            r#"function main() {
                assert(1 < 2);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }
    #[test]
    fn comparison5() {
        let result = compile_and_run(
            r#"function main() {
                assert(1 < 1);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("F".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn divide() {
        let result = compile_and_run(
            r#"function main() {
                assert( 10/2 == 5);
                assert( (10/2) == 5);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("TT".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn multiply() {
        let result = compile_and_run(
            r#"function main() {
                assert( 10*2 == 20);
                assert( (3*2) == 6);
            }"#,
        )
        .expect("Compile an run failed");

        assert_eq!("TT".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn block() {
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
    fn call() {
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
    fn if_1() {
        let result = compile_and_run(
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
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn if_2() {
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
    fn function_assert() {
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
    fn function() {
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
   assert(720 == factorial(6));
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
        let result = compile_and_run(
            r#"function main() {
                    var a = 1;
                    assert(a == 1);
                    a = 0;
                    assert(a == 0);
            }
            "#,
        )
        .expect("Compile an run failed");

        assert_eq!("TT", String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn compile_nested_if() {
        let result = compile_and_run(
            r#"
            function main() {
                if (1) {
                    if (1) {
                        assert(1);
                    } else {
                        assert(0);
                    }
                } else {
                    assert(0);
                }
            }
        "#,
        )
        .expect("Compile and run failed");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn compile_nested_while() {
        let result = compile_and_run(
            r#"
            function main() {
                var i = 0;
                var count = 0;
               
                while (i < 2) {

                    var j = 0;
                    while (j < 3) {
                        count = count + 1;
                        j = j + 1;
                    }
                    j = 0;
                    i = i + 1;
                }
                assert(count == 6);
            }
        "#,
        )
        .expect("Compile and run failed");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn complex_arithmetic() {
        let result = compile_and_run(
            r#"
            function main() {
                assert( ((2 + 3) * 4 )/ 2 + 1 == 11);
            }
        "#,
        )
        .expect("Compile and run failed");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn multiple_vars() {
        let result = compile_and_run(
            r#"
            function main() {
                var a = 1;
                var b = 2;
                var c = 3;
                var d = 4;
                assert(a + b + c + d == 10);
            }
        "#,
        )
        .expect("Compile and run failed");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn recursive_function() {
        let result = compile_and_run(
            r#"
            function sum(n) {
                if (n <= 0) {
                    return 0;
                } else {
                    return n + sum(n - 1);
                }
            }
            
            function main() {
                assert(sum(100) == 5050);
            }
        "#,
        )
        .expect("Compile and run failed");
        assert_eq!("T".to_string(), String::from_utf8(result.stdout).unwrap());
    }

    #[test]
    fn array_literal_assert() {
        let result = compile_and_run(
            r#"
             function main() {
                var x = [1, 2, 3];
                assert(x[0] == 1);
                assert(x[1] == 2);
                assert(x[2] == 3);
                assert(length(x) == 3);
            }
        "#,
        )
        .expect("Compile and run failed");
        assert_eq!(
            "TTTT".to_string(),
            String::from_utf8(result.stdout).unwrap()
        );
    }

    #[test]
    fn array_literal_assert_vars() {
        let result = compile_and_run(
            r#"
             function main() {
                var a = 1;
                var b = 2;
                var c = 3;
                var x = [a, b, c];
                assert(x[0] == 1);
                assert(x[1] == 2);
                assert(x[2] == 3);
                assert(length(x) == 3);
            }
        "#,
        )
        .expect("Compile and run failed");
        assert_eq!(
            "TTTT".to_string(),
            String::from_utf8(result.stdout).unwrap()
        );
    }
}
