use crate::ast::AST;
use crate::ast2::ASTStruct;

peg::parser! {
  grammar lang_parser() for str {

    rule _ = [' ' | '\n']*

    pub rule Number() -> AST
      = n:$(['0'..='9']+) { AST::Number(n.parse().unwrap()) }

    pub rule Id() -> AST
      = n:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | '_' |  'A'..='Z' | '0'..='9' ]*) { AST::Id(n.to_string()) }


    pub rule args() -> Vec<AST>
      = expression() ** (_ "," _)


    ///
    ///
    pub rule call() -> AST
      = n:Id() _ "(" _ a:args() _ ")" {
      AST::Call {
        callee: n.to_string(),
        args:a.iter().cloned().collect()
      }
    }
   ///
   /// INFIX
   rule sum() -> AST
        = l:product() _ op:$("+" / "-") _ r:product() {
             let ast = match op {
                "+" =>  AST::Add{left:Box::new(l), right:Box::new(r)},
                "-" =>  AST::Subtract{left:Box::new(l), right:Box::new(r)},
                 x => panic!("sum found op {}", x)
            };
            ast
        } / product()

    rule product() -> AST
        = l:comparison() _ op:$("*" / "/") _ r:comparison() {
            let ast = match op {
                "*" =>  AST::Multiply{left:Box::new(l), right:Box::new(r)},
                "/" =>  AST::Divide{left:Box::new(l), right:Box::new(r)},
                 x => panic!("product found op {}", x)
            };
            ast

        }
        / comparison()


        rule comparison() -> AST
            = l:atom() _ op:$("==" / "!=") _ r:atom() {
                let ast = match op {
                    "==" =>  AST::Equal{left:Box::new(l), right:Box::new(r)},
                    "!=" =>  AST::NotEqual{left:Box::new(l), right:Box::new(r)},
                x => panic!("comparison found op {}", x)
                };
                ast

            } / unary()

    rule unary() -> AST
      = n:("!")? a:atom() {
        match n {
          Some(term) => AST::Not(Box::new(a)),
          None => a
      }
    }
    rule atom() -> AST
      = call() / Id() / Number() / "("  e:sum() ")" { e }

    pub rule expression() -> AST
            = sum()

    ///
    /// Statements
    ///
    rule returnStmt() -> AST
            = RETURN()  _ e:expression() _ ";" {
            AST::Return {
                term: Box::new(e)
            }
        }

    rule exprStmt() -> AST
            = e:expression() _ ";" { e }

   rule ifStmt() -> AST
            = IF()  _ "(" _ conditional: expression()  _ ")"  _ consequence: statement() _ ELSE()  _ alternative: statement() {
            AST::IfNode {
                conditional: Box::new(conditional),
                consequence: Box::new(consequence),
                alternative: Box::new(alternative)
            }
        }


   rule whileStmt() -> AST
            = WHILE()  _ "(" _ conditional: expression()  _ ")"  _ body: statement() _{
            AST::Wile {
                conditional: Box::new(conditional),
                body: Box::new(body)
            }
        }

   rule varStmt() -> AST
            = VAR()  _ id:Id() _ ASSIGN() _ value:expression() ";" _{
            if let AST::Id(name) = id {
                AST::Var {
                    name: name.clone(),
                    value: Box::new(value)
                }
            }  else {
                unreachable!()
            }
        }
   rule assignmentStmt() -> AST
            =  _ id:Id() _ ASSIGN() _ value:expression() ";" _ {
              if let AST::Id(name) = id {
                AST::Assign {
                    name: name.clone(),
                    value: Box::new(value)
                }
            }  else {
                unreachable!()
            }
        }
   rule blockStmt() -> AST
            =  "{" _  statements:statement()* _ "}"{
            AST::Block {
                statements: statements.iter().cloned().collect()
            }
        }
    rule parameters() -> Vec<String>
      = ids:Id() ** (_ "," _) {
            ids.iter().cloned().map(|item|  {
                 if let AST::Id(name) = item {
                     name
                    } else {
                     unreachable!()
                }
            }).collect()
        }

   rule functionStmt() -> AST
            =  FUNCTION() _ id: Id() _ "(" _ p: parameters() _ ")" _ body:blockStmt() {
               if let AST::Id(name) = id {

                AST::Function {
                    name: name.clone(),
                    parameters: p,
                    body: Box::new(body)
                }
                } else {
                     unreachable!()
                }
    }

        rule statement() -> AST
        = returnStmt() / ifStmt() / whileStmt() / varStmt() / assignmentStmt() / blockStmt() / functionStmt() / exprStmt()

    ///
    /// keywords
    ///
    ///
     pub rule ASSIGN() = "="
    pub rule FUNCTION() = "function"

    pub rule IF() = "if"

    pub rule ELSE() = "else"

    pub rule RETURN() = "return"

    pub rule VAR() = "var"

    pub rule WHILE() = "while"



  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    #[test]
    fn number() {
        assert_eq!(
            AST::Number(1233),
            lang_parser::expression("1233").expect("Parser failed")
        );
    }

    #[test]
    fn id() {
        assert_eq!(
            AST::Id("varname".to_string()),
            lang_parser::expression("varname").expect("Parser failed")
        );
        assert_eq!(
            AST::Id("varname2".to_string()),
            lang_parser::expression("varname2").expect("Parser failed")
        );
        assert_eq!(
            AST::Id("var_name2".to_string()),
            lang_parser::expression("var_name2").expect("Parser failed")
        );
    }

    #[test]
    fn arithmetic1() {
        let expected_ast = AST::Add {
            left: Box::new(AST::Number(1)),
            right: Box::new(AST::Multiply {
                left: Box::new(AST::Number(3)),
                right: Box::new(AST::Number(2)),
            }),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("1+3*2").expect("Parser failed")
        )
    }

    #[test]
    fn arithmetic_wvars() {
        let expected_ast = AST::Add {
            left: Box::new(AST::Id("a".to_string())),
            right: Box::new(AST::Multiply {
                left: Box::new(AST::Id("b".to_string())),
                right: Box::new(AST::Id("c".to_string())),
            }),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("a+b*c").expect("Parser failed")
        )
    }

    #[test]
    fn arithmetic2() {
        let expected_ast = AST::Multiply {
            left: Box::new(AST::Add {
                left: Box::new(AST::Number(1)),
                right: Box::new(AST::Number(3)),
            }),
            right: Box::new(AST::Number(2)),
        };
        println!("{}", expected_ast);

        assert_eq!(
            expected_ast,
            lang_parser::expression("(1+3) * 2").expect("Parser failed")
        );
    }

    #[test]
    fn call_simple() {
        let expected_ast = AST::Call {
            callee: "fname".to_string(),
            args: vec![],
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("fname()").expect("Parser failed")
        );
    }

    #[test]
    fn args_w_whitespace() {
        let expected_ast = vec![AST::Id("a".to_string()), AST::Id("b".to_string())];
        assert_eq!(
            expected_ast,
            lang_parser::args("a, b").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::args("a , b").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::args("a ,b").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::args("a   ,b").expect("Parser failed")
        );
    }
    #[test]
    fn call_w_args_vars() {
        let expected_ast = AST::Call {
            callee: "fname".to_string(),
            args: vec![
               AST::Id("a".to_string()),
               AST::Id("b".to_string()),
            ],
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("fname (a,b)").expect("Parser failed")
        );
    }

    #[test]
    fn call_w_args() {
        let expected_ast = AST::Call {
            callee: "myFunction".to_string(),
            args: vec![
                AST::Add {
                    left: Box::new(AST::Number(1)),
                    right: Box::new(AST::Number(1)),
                },
                AST::Id("a".to_string()),
            ],
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("myFunction(1+1,a)").expect("Parser failed")
        );
    }
}
