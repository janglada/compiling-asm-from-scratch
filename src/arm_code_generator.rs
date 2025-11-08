use crate::ast::AST;
use crate::emitter::Environment;
use crate::visitor::{AstVisitor, Visitor};
use std::collections::{HashMap, LinkedList};
use std::io::Write;

struct ArmCodeGenerator {
    writer: &'static mut (dyn Write),
    env: LinkedList<Environment>,
    pub(crate) locals: HashMap<String, isize>,
    pub(crate) next_local_offset: isize,
    label_counter: i16,
}

impl ArmCodeGenerator {
    fn visit_infix_operands(&mut self, left: &Box<AST>, right: &Box<AST>) -> () {
        left.visit(self);
        writeln!(self.writer, "\tpush {{r0, ip}}");
        right.visit(self);
        writeln!(self.writer, "\tpop {{r1, ip}}");
    }
    fn emit_fn_prologue(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "\tpush {{fp, lr}}")?;
        writeln!(self.writer, "\tmov fp, sp")?;
        writeln!(self.writer, "\tpush {{r0, r1, r2, r3}}")
    }
    fn emit_fn_epilogue(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "\tmov sp, fp")?;
        // .We set r0 and thus our return value to 0.
        // This is to mimic the fact that JavaScript functions return undefined
        // when there’s no explicit return
        writeln!(self.writer, "\tmov r0, #0")?;
        writeln!(self.writer, "\tpop {{ fp, pc }}")
    }
    fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!(".L{}", self.label_counter)
    }
}
impl Visitor<()> for ArmCodeGenerator {
    fn visit_assert(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Assert(condition) = node else {
            panic!("Expected Assert node, got: {:?}", node)
        };
        condition.visit(self)?;
        write!(self.writer, "\t")?;
        writeln!(
            self.writer,
            r#"cmp r0, #1
            moveq r0, #'T'
            movne r0, #'F'
            bl putchar"#
        )
    }

    fn visit_array_length(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::ArrayLength(array) = node else {
            panic!("Expected Assert node, got: {:?}", node)
        };
        array.visit(self)?;
        writeln!(self.writer, "\tldr r0, [r0, #0]")
    }

    fn visit_array_lookup(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::ArrayLookup { array, index } = node else {
            panic!("Expected Assert node, got: {:?}", node)
        };
        array.visit(self)?;
        writeln!(self.writer, "\tpush {{r0, ip}}");
        index.visit(self)?;
        writeln!(self.writer, "\tpop {{r1, ip}}");
        writeln!(self.writer, "\tldr r2, [r1]");
        writeln!(self.writer, "\tcmp r0, r2");
        writeln!(self.writer, "\tmovhs r0, #0");
        writeln!(self.writer, "\taddlo r1, r1, #4");
        writeln!(self.writer, "\tlsllo r0, r0, #2");
        writeln!(self.writer, "\tldrlo r0, [r1, r0]")
    }

    fn visit_array_literal(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::ArrayLiteral(array_items) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };
        let len = array_items.len();
        writeln!(self.writer, "\tldr r0, ={}", 4 * (len + 1));
        writeln!(self.writer, "\tbl malloc");
        writeln!(self.writer, "\tpush {{r4, ip}}");
        writeln!(self.writer, "\tmov r4, r0");
        writeln!(self.writer, "\tldr r0, ={}", len);
        writeln!(self.writer, "\tstr r0, [r4]");
        for (i, item) in array_items.iter().enumerate() {
            item.visit(self)?;
            writeln!(self.writer, "\tstr r0, [r4, #{}]", 4 * (i + 1));
        }

        writeln!(self.writer, "\tmov r0, r4");
        writeln!(self.writer, "\tpop {{r4, ip}}")
    }

    fn visit_boolean(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Boolean(value) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };
        writeln!(self.writer, "\tmov r0, #{}", if *value { 1 } else { 0 })
    }

    fn visit_number(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Number(number) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };
        writeln!(self.writer, "\tldr r0, ={}", *number)
    }

    fn visit_id(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Id(name) = node else {
            panic!("Expected ArrayLiteral node, got: {:?}", node)
        };
        let env = self.env.iter().last().expect("Missing environment");
        let offset = env
            .locals
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {}", name));
        writeln!(self.writer, "\tldr r0, [fp, #{}]", offset)
    }

    fn visit_not(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Not(term) = node else {
            panic!("Expected Not node, got: {:?}", node)
        };
        term.visit(self)?;
        writeln!(
            self.writer,
            r#" cmp r0, #0
    moveq r0, #1
    movne r0, #0"#
        )
    }

    fn visit_equal(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Equal { left, right } = node else {
            panic!("Expected Not node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(
            self.writer,
            r#"cmp r0, r1
    moveq r0, #1
    movne r0, #0"#
        )
    }

    fn visit_not_equal(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::NotEqual { left, right } = node else {
            panic!("Expected NotEqual node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(
            self.writer,
            r#"cmp r0, r1
    moveq r0, #0
    movne r0, #1"#
        )
    }

    fn visit_add(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Add { left, right } = node else {
            panic!("Expected NotEqual node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(self.writer, "add r0, r1, r0")
    }

    fn visit_subtract(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Subtract { left, right } = node else {
            panic!("Expected NotEqual node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(self.writer, "sub r0, r1, r0")
    }

    fn visit_multiply(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Multiply { left, right } = node else {
            panic!("Expected Multiply node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(self.writer, "sub r0, r0, r1")
    }

    fn visit_divide(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Divide { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(self.writer, "udiv r0, r1, r0")
    }

    fn visit_less_than(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::LessThan { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(
            self.writer,
            r#"cmp r1, r0
    movlt r0, #1
    movge r0, #0"#
        )
    }

    fn visit_greater_than(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::GreaterThan { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(
            self.writer,
            r#"cmp r1, r0
    movgt r0, #1
    movle r0, #0"#
        )
    }

    fn visit_less_than_equal(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::LessThanEqual { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(
            self.writer,
            r#"cmp r1, r0
    movle r0, #1
    movgt r0, #0"#
        )
    }

    fn visit_greater_than_equal(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::GreaterThanEqual { left, right } = node else {
            panic!("Expected Divide node, got: {:?}", node)
        };
        self.visit_infix_operands(left, right);
        write!(self.writer, "\t")?;
        writeln!(
            self.writer,
            r#"cmp r1, r0
    movge r0, #1
    movlt r0, #0"#
        )
    }

    fn visit_call(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Call { args, callee } = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        let len = args.len();
        if args.is_empty() {
            writeln!(self.writer, "\tbl {}", callee)
        } else if len == 1 {
            args.first().unwrap().visit(self)?;
            writeln!(self.writer, "\tbl {}", callee)
        } else if (2..=4).contains(&len) {
            // allocate enough stack space for up to four arguments (16 bytes)
            // We do that by subtracting from the stack
            // pointer since the stack grows from higher memory addresses to
            // lower.
            writeln!(self.writer, "\tsub sp, sp, #16")?;
            args.iter().enumerate().for_each(|(i, arg)| {
                arg.visit(self)?;
                // We multiply by four to convert array indexes 0, 1, 2, 3 into
                // stack offsets in bytes: 0, 4, 8, 12.
                writeln!(self.writer, "\tstr r0, [sp, #{}]", 4 * i).expect("Write failed");
            });
            writeln!(self.writer, "\tpop {{r0, r1, r2, r3}}")?;
            writeln!(self.writer, "\tbl {}", callee)
        } else {
            panic!("More than 4 arguments are not supported in function calls")
        }
    }

    fn visit_return(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Return { term } = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        term.visit(self)?;
        writeln!(self.writer, "\tmov sp, fp")?;
        writeln!(self.writer, "\tpop {{fp, pc}}")
    }

    fn visit_block(&mut self, node: &AST) -> std::io::Result<()> {
        let AST::Block(statements) = node else {
            panic!("Expected Call node, got: {:?}", node)
        };
        for statement in statements {
            statement.visit(self)?;
        }
        Ok(())
    }

    fn visit_if(&mut self, node: &AST) -> std::io::Result<()> {
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
        conditional.visit(self)?;
        writeln!(self.writer, "\tcmp r0, #0")?;
        consequence.visit(self)?;
        writeln!(self.writer, "\tb {}", end_if_label)?;
        writeln!(self.writer, "{}:", if_false_label)?;
        alternative.visit(self)?;
        writeln!(self.writer, "{}:", end_if_label)
    }

    fn visit_function(&mut self, node: &AST) -> std::io::Result<()> {
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
        writeln!(self.writer)?;
        writeln!(self.writer, ".global {}", name)?;
        writeln!(self.writer, "{}:", name)?;
        self.emit_fn_prologue()?;

        let mut locals: HashMap<String, isize> = HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            locals.insert(parameter.clone(), 4 * i as isize - 16);
        }
        // let env = Environment {
        //     locals,
        //     next_local_offset: -20,
        // };
        let mut code_gen_visitor = ArmCodeGenerator {
            writer: self.writer,
            env: Default::default(),
            label_counter: 0,
            locals,
            next_local_offset: -20,
        };
        body.visit(&mut code_gen_visitor)?;
        // self.env.push_back(env);
        // self.write(body, writer)?;
        // self.env.pop_back();
        self.emit_fn_epilogue();
        Ok(())
    }

    fn visit_var(&mut self, node: &AST) -> std::io::Result<()> {
        todo!()
    }

    fn visit_assign(&mut self, node: &AST) -> std::io::Result<()> {
        todo!()
    }

    fn visit_while(&mut self, node: &AST) -> std::io::Result<()> {
        todo!()
    }

    fn visit_undefined(&mut self, node: &AST) -> std::io::Result<()> {
        todo!()
    }

    fn visit_null(&mut self, node: &AST) -> std::io::Result<()> {
        todo!()
    }

    fn visit_main(&mut self, node: &AST) -> std::io::Result<()> {
        todo!()
    }
}
