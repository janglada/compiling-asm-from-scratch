use crate::ast::AST;
use crate::error::CompileError;

peg::parser! {
  pub grammar lang_parser() for str {

    rule _ = quiet!{[' ' | '\n' | '\t']*}


    pub rule Number() -> AST
      = n:$(['0'..='9']+) { AST::Number(n.parse().unwrap()) }

    pub rule Id() -> AST
      = n:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | '_' |  'A'..='Z' | '0'..='9' ]*) { AST::Id(n.to_string()) }

    pub rule True() -> AST
      = "true" { AST::Boolean(true) }

    pub rule False() -> AST
      = "false" { AST::Boolean(false) }

    pub rule Null() -> AST
      = "null" { AST::Null }

    pub rule Undefined() -> AST
      = "undefined" { AST::Undefined }

    pub rule args() -> Vec<AST>
      = expression() ** (_ "," _)

    pub rule ArrayLiteral() -> AST
      =   "[" _ a: args() _ "]" { AST::ArrayLiteral(a) }

    pub rule ArrayLookup() -> AST
        = id:Id() _ "[" _ e:expression() _ "]" { AST::ArrayLookup {array: Box::new(id),index: Box::new(e)} }

    ///
    ///
    pub rule call() -> AST
      = callee:Id() _ "(" _ a:args() _ ")" {
            if callee.to_string() == "assert" {
                let mut iter = a.into_iter().take(1);
                let ast: AST = iter.next().unwrap();
                AST::Assert(Box::new(ast))
            } else if callee.to_string() == "length" {
                let mut iter = a.into_iter().take(1);
                let ast: AST = iter.next().unwrap();
                AST::ArrayLength(Box::new(ast))
            }else if callee.to_string() == "print" {
                let mut iter = a.into_iter().take(1);
                let ast: AST = iter.next().unwrap();
                AST::Print(Box::new(ast))
            } else {
              AST::Call {
                callee: callee.to_string(),
                args:a.to_vec()
              }
            }
    }

    pub rule atom() -> AST
      = call() / ArrayLiteral() / ArrayLookup() /  True() / False() / Null() / Undefined() / Id() / Number()

    /// allow whitespaces before after
    pub rule expression() -> AST = _ e:expressionPrecedence() _ {e }

    pub rule expressionPrecedence() -> AST = precedence!{
        x:(@) _ "==" _ y:@ { AST::Equal{left:x.into(), right:y.into()} }
        x:(@) _ "!=" _ y:@ { AST::NotEqual{left:x.into(), right:y.into()} }
         --
        x:(@) _ ">=" _ y:@ { AST::GreaterThanEqual{left:x.into(), right:y.into()} }
        x:(@) _ ">" _ y:@ { AST::GreaterThan{left:x.into(), right:y.into()} }
        x:(@) _ "<=" _ y:@ { AST::LessThanEqual{left:x.into(), right:y.into()} }
        x:(@) _ "<" _ y:@ { AST::LessThan{left:x.into(), right:y.into()} }
        --
        x:(@) _ "+" _ y:@ { AST::Add{left:x.into(), right:y.into()} }
        x:(@) _ "-" _ y:@ { AST::Subtract{left:x.into(), right:y.into()} }
        --
        x:(@) _ "*" _ y:@ { AST::Multiply{left:x.into(), right:y.into()} }
        x:(@) _ "/" _ y:@ { AST::Divide{left:x.into(), right:y.into()} }
        --
       "!" _ x:@ { AST::Not(x.into()) }
        --
        "(" _ v:expression() _ ")" { v }
        n :atom() {n}
    }

    ///
    /// Statements
    ///
    pub rule returnStmt() -> AST
            = RETURN()  _ e:expression() _ ";" {
            AST::Return {
                term: e.into()
            }
        }

   pub rule exprStmt() -> AST
            = e:expression() _ ";" _  { e }

   pub  rule ifStmt() -> AST
            = "if"  _ "(" _ conditional: expression()  _ ")"  _ consequence: statement() _ ELSE()  _ alternative: statement() _ {
            AST::IfNode {
                conditional: conditional.into(),
                consequence: consequence.into(),
                alternative: alternative.into()
            }
        }


   pub rule whileStmt() -> AST
            = WHILE()   "(" _ conditional: expression()  _ ")"  _   body: statement() _ {
            AST::While {
                conditional: conditional.into(),
                body: body.into()
            }
        }

  pub rule varStmt() -> AST
            = VAR()   id:Id()  ASSIGN()  value:expression() _ ";" _{
            if let AST::Id(name) = id {
                AST::Var {
                    name,
                    value: value.into()
                }
            }  else {
                unreachable!()
            }
        }
   pub rule assignmentStmt() -> AST
            =  id:Id()  ASSIGN()  value:expression() _ ";" _ {
              if let AST::Id(name) = id {
                AST::Assign {
                    name,
                    value: value.into()
                }
            }  else {
                unreachable!()
            }
        }
   pub rule blockStmt() -> AST
            =  "{" _  statements:statement()* _ "}" {
            AST::Block(statements.to_vec())
        }
    pub rule parameters() -> Vec<String>
      = ids:Id() ** (_ "," _) {
            ids.iter().cloned().map(|item|  {
                 if let AST::Id(name) = item {
                     name
                    } else {
                     unreachable!()
                }
            }).collect()
        }

   pub rule functionStmt() -> AST
            =  FUNCTION() _ id: Id() _ "(" _ p: parameters() _ ")" _ body:blockStmt() _ {
            if let AST::Id(name) = id {

                // if false && name == "main" {
                //   if let AST::Block(statements) = body {
                //     AST::Main(statements)
                //   } else {
                //         unreachable!()
                //    }
                // } else {

                    AST::Function {
                        name,
                        parameters: p,
                        body: body.into()
                    }
                //}


            } else {
                 unreachable!()
            }
    }

   pub rule statement() -> AST
        = returnStmt() / ifStmt() / whileStmt() / varStmt() / assignmentStmt() / blockStmt() / functionStmt() / exprStmt()

   pub rule parser() -> AST
        = _ s:statement() ** _ {
            if s.len() == 1 {

                    return s.first().unwrap().clone();

            }
                AST::Block(s)
        }


    ///
    /// keywords
    ///
    ///
    pub rule ASSIGN() = _ "=" _

    pub rule FUNCTION() = "function"

    pub rule IF() = "if"

    pub rule ELSE() = "else"

    pub rule RETURN() = "return"

    pub rule VAR() = "var" " "

    pub rule WHILE() = "while" _



  }
}

pub fn parse(input: &str) -> Result<AST, CompileError> {
    lang_parser::parser(input).map_err(CompileError::ParseError)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            left: 1.into(),
            right: AST::Multiply {
                left: 3.into(),
                right: 2.into(),
            }
            .into(),
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
            left: "a".to_string().into(),
            right: AST::Multiply {
                left: "b".to_string().into(),
                right: "c".to_string().into(),
            }
            .into(),
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
            left: AST::Add {
                left: 1.into(),
                right: 3.into(),
            }
            .into(),
            right: 2.into(),
        };
        println!("{}", expected_ast);

        assert_eq!(
            expected_ast,
            lang_parser::expression("(1+3) * 2").expect("Parser failed")
        );
    }

    #[test]
    fn infix_sums() {
        lang_parser::expression("1 + 2 +3 +4+5").expect("Parser failed");
    }

    #[test]
    fn infix_products() {
        lang_parser::expression("1 * 2 *3 *4*5").expect("Parser failed");
    }

    #[test]
    fn infix_parse2() {
        lang_parser::expression("4 + 2 * 10 + 3 * 6").expect("Parser failed");
        // lang_parser::expression("42 == 4 + 2 * (12 - 2) + 3 * (5 + 1)").expect("Parser failed");
    }

    #[test]
    fn infix_comparison1() {
        lang_parser::expression("a > 1").expect("Parser failed");
    }

    #[test]
    fn infix_comparison2() {
        lang_parser::expression("a < 1").expect("Parser failed");
    }

    #[test]
    fn infix_comparison3() {
        lang_parser::expression("a >= 1").expect("Parser failed");
    }

    #[test]
    fn infix_comparison4() {
        lang_parser::expression("a <= 1").expect("Parser failed");
    }

    #[test]
    fn substract() {
        let expected_ast = AST::Subtract {
            left: 12.into(),
            right: 2.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("12 - 2").expect("Parser failed")
        );
    }

    #[test]
    fn substract_and_equal() {
        let expected_ast = AST::Equal {
            left: 2.into(),
            right: AST::Subtract {
                left: 3.into(),
                right: 1.into(),
            }
            .into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression(" 2 ==  3 -  1 ").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::expression("2 ==3-1 ").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::expression(" 2 ==3-1").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::statement("2 ==  3 -  1;").expect("Parser failed")
        );
    }

    #[test]
    fn infix() {
        let expected_ast = AST::Equal {
            left: 42.into(),
            right: AST::Add {
                left: AST::Add {
                    left: 4.into(),
                    right: AST::Multiply {
                        left: 2.into(),
                        right: AST::Subtract {
                            left: 12.into(),
                            right: 2.into(),
                        }
                        .into(),
                    }
                    .into(),
                }
                .into(),
                right: AST::Multiply {
                    left: 3.into(),
                    right: AST::Add {
                        left: 5.into(),
                        right: 1.into(),
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
        };
        println!("{}", expected_ast);

        assert_eq!(
            expected_ast,
            lang_parser::expression("42 == 4 + 2 * (12 - 2) + 3 * (5 + 1)").expect("Parser failed")
        );
        // 42 == 4 +(2+(12-2) + (3*(5+1))
        // 42 == (4+2*(12-2) + 3*(5+1)
    }

    ///
    ///
    #[test]
    fn comparison() {
        let expected_ast = AST::NotEqual {
            left: 1.into(),
            right: 2.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("1 != 2").expect("Parser failed")
        )
    }

    #[test]
    fn comparison_ast1() {
        let expected_ast = AST::GreaterThan {
            left: 1.into(),
            right: 2.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("1 > 2").expect("Parser failed")
        )
    }

    #[test]
    fn comparison_ast2() {
        let expected_ast = AST::GreaterThan {
            left: 1.into(),
            right: 2.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("1 >2").expect("Parser failed")
        )
    }
    #[test]
    fn comparison_ast3() {
        let expected_ast = AST::LessThan {
            left: 1.into(),
            right: 2.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("1 <2").expect("Parser failed")
        )
    }

    #[test]
    fn comparison_ast4() {
        let expected_ast = AST::LessThanEqual {
            left: 1.into(),
            right: 2.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("1<=2").expect("Parser failed")
        )
    }
    ///
    ///
    #[test]
    fn comparison_expr() {
        let expected_ast = AST::Equal {
            left: AST::Add {
                left: 1.into(),
                right: 1.into(),
            }
            .into(),
            right: AST::Subtract {
                left: 2.into(),
                right: 1.into(),
            }
            .into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("1 + 1 == 2 -1").expect("Parser failed")
        )
    }

    #[test]
    fn comparison_expr2() {
        let expected_ast = AST::Equal {
            left: 6.into(),
            right: AST::Add {
                left: 4.into(),
                right: AST::Subtract {
                    left: 3.into(),
                    right: 1.into(),
                }
                .into(),
            }
            .into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("6 == 4 + (3-1) ").expect("Parser failed")
        )
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
            args: vec![AST::Id("a".to_string()), AST::Id("b".to_string())],
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
                    left: AST::Number(1).into(),
                    right: AST::Number(1).into(),
                },
                AST::Id("a".to_string()),
                AST::NotEqual {
                    left: 1.into(),
                    right: 0.into(),
                },
            ],
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("myFunction(1+1,a, 1 != 0)").expect("Parser failed")
        );
    }

    #[test]
    fn var_assign() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::Number(1).into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("var a = 1;").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::parser("var a=1;").expect("Parser failed")
        );
    }
    #[test]
    fn var_assign_bool() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::Boolean(true).into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("var a = true;").expect("Parser failed")
        );
    }

    #[test]
    fn var_assign_null() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::Null.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("var a = null;").expect("Parser failed")
        );
    }
    #[test]
    fn var_assign_empty_array() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::ArrayLiteral(vec![]).into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("var a = [];").expect("Parser failed")
        );
    }

    #[test]
    fn var_assign_non_empty_array() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::ArrayLiteral(vec![AST::Number(1)]).into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("var a = [1];").expect("Parser failed")
        );
    }

    #[test]
    fn var_assign_undefined() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::Undefined.into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("var a = undefined;").expect("Parser failed")
        );
    }

    #[test]
    fn assign() {
        let expected_ast = AST::Assign {
            name: "a".to_string(),
            value: AST::Number(1).into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("a = 1;").expect("Parser failed")
        );
        assert_eq!(
            expected_ast,
            lang_parser::parser("a=1;").expect("Parser failed")
        );
    }

    #[test]
    fn assign_and_use() {
        let expected_ast = AST::Block(vec![
            AST::Var {
                name: "x".to_string(),
                value: AST::Number(1).into(),
            },
            AST::Var {
                name: "y".to_string(),
                value: AST::Number(2).into(),
            },
            AST::Var {
                name: "z".to_string(),
                value: AST::Add {
                    left: AST::Id("x".into()).into(),
                    right: AST::Id("y".into()).into(),
                }
                .into(),
            },
            AST::Assert(
                AST::Equal {
                    left: AST::Id("z".into()).into(),
                    right: AST::Number(3).into(),
                }
                .into(),
            ),
        ]);
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"{
                    var x = 1;
                    var y = 2;
                    var z = x + y;
                     assert(z == 3);
             }"#
            )
            .expect("Parser failed")
        );
    }

    //
    #[test]
    fn return_stmt() {
        let expected_ast = AST::Return {
            term: AST::Add {
                left: AST::Number(1).into(),
                right: AST::Number(1).into(),
            }
            .into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("return 1 + 1;").expect("Parser failed")
        );
        let expected_ast = AST::Return {
            term: AST::Id("a".to_string()).into(),
        };
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser("return   a;").expect("Parser failed")
        );
    }

    #[test]
    fn block_stmt() {
        let expected_ast = AST::Block(vec![
            AST::Var {
                name: "a".to_string(),
                value: AST::Number(1).into(),
            },
            AST::Var {
                name: "b".to_string(),
                value: AST::Number(2).into(),
            },
        ]);
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"{
                var a = 1;
                var b = 2;
             }"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn block_stmt_within_main() {
        let expected_ast = AST::Function {
            name: "main".to_string(),
            parameters: vec![],
            body: AST::Block(vec![AST::Block(vec![
                AST::Assert(1.into()),
                AST::Assert(1.into()),
                AST::Call {
                    callee: "putchar".into(),
                    args: vec![AST::Number(84)],
                },
            ])])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"function main() {
                    {
                        assert(1);
                        assert(1);
                        putchar(84);
                    }
             }"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn block_stmt_within_main2() {
        let expected_ast = AST::Function {
            name: "main".to_string(),
            parameters: vec![],
            body: AST::Block(vec![AST::Block(vec![
                AST::IfNode {
                    conditional: 0.into(),
                    consequence: AST::Block(vec![AST::Assert(0.into())]).into(),
                    alternative: AST::Block(vec![AST::Assert(1.into())]).into(),
                },
                AST::Call {
                    callee: "putchar".into(),
                    args: vec![AST::Number(84)],
                },
            ])])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
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
            .expect("Parser failed")
        );
    }

    #[test]
    fn if_stmt() {
        let expected_ast = AST::IfNode {
            conditional: AST::Id("a".to_string()).into(),
            consequence: AST::Block(vec![AST::Assign {
                name: "a".to_string(),
                value: AST::Number(1).into(),
            }])
            .into(),
            alternative: AST::Block(vec![AST::Assign {
                name: "a".to_string(),
                value: AST::Number(0).into(),
            }])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"if (a) {
               a = 1;
            } else {
                a = 0;
            }"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn while_stmt() {
        let expected_ast = AST::While {
            conditional: AST::Id("a".into()).into(),
            body: AST::Block(vec![
                AST::Assign {
                    name: "b".to_string(),
                    value: AST::Number(1).into(),
                },
                AST::Assign {
                    name: "a".to_string(),
                    value: AST::Number(2).into(),
                },
            ])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"while(a) {
                b = 1;
                a = 2;
            }"#
            )
            .expect("Parser failed")
        );
    }
    #[test]
    fn while_cond() {
        let expected_ast = AST::Block(vec![
            AST::Var {
                name: "a".to_string(),
                value: AST::Number(1).into(),
            },
            AST::While {
                conditional: AST::NotEqual {
                    left: AST::Id("a".to_string()).into(),
                    right: AST::Number(10).into(),
                }
                .into(),
                body: AST::Block(vec![AST::Assign {
                    name: "a".to_string(),
                    value: AST::Add {
                        left: AST::Id("a".to_string()).into(),
                        right: AST::Number(1).into(),
                    }
                    .into(),
                }])
                .into(),
            },
        ]);

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"
                var a = 1;
                while(a != 10) {
                a = a+1;
            }"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn main_while_cond() {
        let expected_ast = AST::Function {
            name: "main".to_string(),
            parameters: vec![],
            body: AST::Block(vec![
                AST::Var {
                    name: "a".to_string(),
                    value: AST::Number(1).into(),
                },
                AST::While {
                    conditional: AST::NotEqual {
                        left: AST::Id("a".to_string()).into(),
                        right: AST::Number(10).into(),
                    }
                    .into(),
                    body: AST::Block(vec![AST::Assign {
                        name: "a".to_string(),
                        value: AST::Add {
                            left: AST::Id("a".to_string()).into(),
                            right: AST::Number(1).into(),
                        }
                        .into(),
                    }])
                    .into(),
                },
            ])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"function main() {
                    var a = 1;
                    while(a != 10) {
                    a = a+1;
                    }
                }"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn function_stmt() {
        let expected_ast = AST::Function {
            name: "myFunc".to_string(),
            body: AST::Block(vec![
                AST::Assign {
                    name: "b".to_string(),
                    value: AST::Number(1).into(),
                },
                AST::While {
                    conditional: AST::Id("a".into()).into(),
                    body: AST::Block(vec![AST::Assign {
                        name: "b".to_string(),
                        value: AST::Number(1).into(),
                    }])
                    .into(),
                },
                AST::Return {
                    term: AST::Id("a".to_string()).into(),
                },
            ])
            .into(),
            parameters: vec![String::from("a"), String::from("b")],
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"function myFunc(a, b) {
                b = 1;
                while (a) {
                    b = 1;
                }
                return a;
            }"#
            )
            .expect("Parser failed")
        );
    }
    #[test]
    fn main_func() {
        let expected_ast = AST::Function {
            name: "main".to_string(),
            body: AST::Block(vec![AST::Var {
                name: "x".to_string(),
                value: AST::Number(1).into(),
            }
            .into()])
            .into(),
            parameters: vec![],
        };

        // let expected_ast = AST::Main(vec![AST::Var {
        //     name: "x".to_string(),
        //     value: AST::Number(1).into(),
        // }]);

        let ast = lang_parser::parser(
            r#"function main() {
var x = 1;
}"#,
        )
        .expect("Parser failed");

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"    function main() {
var x = 1;
}"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn factorial() {
        let expected_ast = AST::Function {
            name: "factorial".to_string(),
            parameters: vec![String::from("n")],

            body: AST::Block(vec![
                AST::Var {
                    name: "result".to_string(),
                    value: AST::Number(1).into(),
                },
                AST::While {
                    conditional: AST::NotEqual {
                        left: AST::Id(String::from("n")).into(),
                        right: AST::Number(1).into(),
                    }
                    .into(),
                    body: AST::Block(vec![
                        AST::Assign {
                            name: "result".to_string(),
                            value: AST::Multiply {
                                left: AST::Id(String::from("result")).into(),
                                right: AST::Id(String::from("n")).into(),
                            }
                            .into(),
                        },
                        AST::Assign {
                            name: "n".to_string(),
                            value: AST::Subtract {
                                left: AST::Id(String::from("n")).into(),
                                right: AST::Number(1).into(),
                            }
                            .into(),
                        },
                    ])
                    .into(),
                },
                AST::Return {
                    term: AST::Id("result".to_string()).into(),
                },
            ])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"function factorial(n) {
    var result = 1;
    while (n != 1) {
        result = result * n;
        n = n - 1;
    }
    return result; 
}"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn main_stmt() {
        let expected_ast = AST::Function {
            name: "main".to_string(),
            parameters: vec![],
            body: AST::Block(vec![
                AST::Assign {
                    name: "b".to_string(),
                    value: AST::Number(1).into(),
                },
                AST::While {
                    conditional: AST::Id("a".into()).into(),
                    body: AST::Block(vec![AST::Assign {
                        name: "b".to_string(),
                        value: AST::Number(1).into(),
                    }])
                    .into(),
                },
            ])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"function main() {
                b = 1;
                while (a) {
                    b = 1;
                }
            }"#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn main_and_other_stmt() {
        let expected_ast = AST::Block(vec![
            AST::Function {
                name: "main".to_string(),
                parameters: vec![],
                body: AST::Block(vec![AST::Var {
                    name: "a".into(),
                    value: 1.into(),
                }])
                .into(),
            },
            AST::Function {
                name: "other".into(),
                parameters: vec![],
                body: AST::Block(vec![AST::Var {
                    name: "b".into(),
                    value: 1.into(),
                }])
                .into(),
            },
        ]);
        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"
                function main() {
                    var a = 1;
            }

            function other() {
                var b = 1;
            }
            "#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn nested_if_statements() {
        let expected_ast = AST::Function {
            name: "main".to_string(),
            parameters: vec![],
            body: AST::Block(vec![AST::IfNode {
                conditional: Box::new(AST::Number(1)),
                consequence: Box::new(AST::Block(vec![AST::IfNode {
                    conditional: Box::new(AST::Number(2)),
                    consequence: Box::new(AST::Block(vec![AST::Assert(Box::new(AST::Number(1)))])),
                    alternative: Box::new(AST::Block(vec![AST::Assert(Box::new(AST::Number(0)))])),
                }])),
                alternative: Box::new(AST::Block(vec![AST::Assert(Box::new(AST::Number(0)))])),
            }])
            .into(),
        };

        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"
            function main() {
                if (1) {
                    if (2) {
                        assert(1);
                    } else {
                        assert(0);
                    }
                } else {
                    assert(0);
                }
            }
        "#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn complex_arithmetic() {
        let expected_ast = AST::Add {
            left: Box::new(AST::Divide {
                left: Box::new(AST::Multiply {
                    left: Box::new(AST::Add {
                        left: Box::new(AST::Number(1)),
                        right: Box::new(AST::Number(2)),
                    }),
                    right: Box::new(AST::Number(3)),
                }),
                right: Box::new(AST::Subtract {
                    left: Box::new(AST::Number(4)),
                    right: Box::new(AST::Number(2)),
                }),
            }),
            right: Box::new(AST::Number(5)),
        };

        assert_eq!(
            expected_ast,
            lang_parser::expression("(1 + 2) * 3 / (4 - 2) + 5").expect("Parser failed")
        );
    }

    ///
    ///  ((2 + 3) * 4 )/ 2 + 1 == 1
    ///
    #[test]
    fn complex_arithmetic2() {
        let expected_ast = AST::Equal {
            left: Box::new(AST::Add {
                left: Box::new(AST::Divide {
                    left: Box::new(AST::Multiply {
                        left: Box::new(AST::Add {
                            left: Box::new(AST::Number(2)),
                            right: Box::new(AST::Number(3)),
                        }),
                        right: Box::new(AST::Number(4)),
                    }),
                    right: Box::new(AST::Number(2)),
                }),
                right: Box::new(AST::Number(1)),
            }),
            right: Box::new(AST::Number(1)),
        };

        assert_eq!(
            expected_ast,
            lang_parser::expression("((2 + 3) * 4 )/ 2 + 1 == 1").expect("Parser failed")
        );
    }

    #[test]
    fn multiple_function_parameters() {
        let expected_ast = AST::Function {
            name: "test".to_string(),
            parameters: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
            body: Box::new(AST::Block(vec![AST::Return {
                term: Box::new(AST::Add {
                    left: Box::new(AST::Add {
                        left: Box::new(AST::Add {
                            left: Box::new(AST::Id("a".to_string())),
                            right: Box::new(AST::Id("b".to_string())),
                        }),
                        right: Box::new(AST::Id("c".to_string())),
                    }),
                    right: Box::new(AST::Id("d".to_string())),
                }),
            }])),
        };

        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"
            function test(a, b, c, d) {
                return a + b + c + d;
            }
        "#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn nested_while_loops() {
        let expected_ast = AST::Function {
            name: "main".to_string(),
            parameters: vec![],
            body: Box::new(AST::Block(vec![
                AST::Var {
                    name: "i".to_string(),
                    value: Box::new(AST::Number(0)),
                },
                AST::While {
                    conditional: Box::new(AST::LessThan {
                        left: Box::new(AST::Id("i".to_string())),
                        right: Box::new(AST::Number(3)),
                    }),
                    body: Box::new(AST::Block(vec![
                        AST::Var {
                            name: "j".to_string(),
                            value: Box::new(AST::Number(0)),
                        },
                        AST::While {
                            conditional: Box::new(AST::LessThan {
                                left: Box::new(AST::Id("j".to_string())),
                                right: Box::new(AST::Number(2)),
                            }),
                            body: Box::new(AST::Block(vec![AST::Assign {
                                name: "j".to_string(),
                                value: Box::new(AST::Add {
                                    left: Box::new(AST::Id("j".to_string())),
                                    right: Box::new(AST::Number(1)),
                                }),
                            }])),
                        },
                        AST::Assign {
                            name: "i".to_string(),
                            value: Box::new(AST::Add {
                                left: Box::new(AST::Id("i".to_string())),
                                right: Box::new(AST::Number(1)),
                            }),
                        },
                    ])),
                },
            ])),
        };

        assert_eq!(
            expected_ast,
            lang_parser::parser(
                r#"
            function main() {
                var i = 0;
                while (i < 3) {
                    var j = 0;
                    while (j < 2) {
                        j = j + 1;
                    }
                    i = i + 1;
                }
            }
        "#
            )
            .expect("Parser failed")
        );
    }

    #[test]
    fn arrayliteral() {
        let expected_ast = AST::ArrayLiteral(vec![
            AST::Number(1),
            AST::Number(2),
            AST::Id("myVar".to_string()),
            AST::Add {
                left: AST::Number(100).into(),
                right: AST::Number(9).into(),
            },
        ]);

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("[1, 2, myVar, 100+9]").expect("Parser failed")
        )
    }

    #[test]
    fn arrayliteral_assign() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::ArrayLiteral(vec![
                // AST::Boolean(true),
                AST::Number(2),
                AST::Id("myVar".to_string()),
                AST::Add {
                    left: AST::Number(100).into(),
                    right: AST::Number(9).into(),
                },
            ])
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::statement("var a = [2, myVar,(100 + 9)];").expect("Parser failed")
        )
    }

    #[test]
    fn arraylookup() {
        let expected_ast = AST::ArrayLookup {
            array: AST::Id("myArray".to_string()).into(),
            index: AST::Number(2).into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("myArray[2]").expect("Parser failed")
        )
    }
    #[test]
    fn arraylookup_index_exp() {
        let expected_ast = AST::ArrayLookup {
            array: AST::Id("myArray".to_string()).into(),
            index: AST::Add {
                left: AST::Number(4).into(),
                right: AST::Number(5).into(),
            }
            .into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::expression("myArray[4+5]").expect("Parser failed")
        )
    }

    #[test]
    fn array_length() {
        let expected_ast = AST::Var {
            name: "a".to_string(),
            value: AST::ArrayLength(AST::Id("b".to_string()).into()).into(),
        };

        println!("{}", expected_ast);
        assert_eq!(
            expected_ast,
            lang_parser::statement("var a =  length(b);").expect("Parser failed")
        )
    }

    // ========== NEW PARSER TESTS ==========

    // ===== Print Statement Tests =====

    #[test]
    fn print_number() {
        let expected_ast = AST::Print(AST::Number(42).into());
        assert_eq!(
            expected_ast,
            lang_parser::statement("print(42);").expect("Parser failed")
        )
    }

    #[test]
    fn print_variable() {
        let expected_ast = AST::Print(AST::Id("x".to_string()).into());
        assert_eq!(
            expected_ast,
            lang_parser::statement("print(x);").expect("Parser failed")
        )
    }

    #[test]
    fn print_expression() {
        let expected_ast = AST::Print(
            AST::Add {
                left: AST::Number(10).into(),
                right: AST::Number(20).into(),
            }
            .into(),
        );
        assert_eq!(
            expected_ast,
            lang_parser::statement("print(10 + 20);").expect("Parser failed")
        )
    }

    #[test]
    fn print_array_access() {
        let expected_ast = AST::Print(
            AST::ArrayLookup {
                array: AST::Id("arr".to_string()).into(),
                index: AST::Number(0).into(),
            }
            .into(),
        );
        assert_eq!(
            expected_ast,
            lang_parser::statement("print(arr[0]);").expect("Parser failed")
        )
    }

    #[test]
    fn print_function_call() {
        let expected_ast = AST::Print(
            AST::Call {
                callee: "foo".to_string(),
                args: vec![AST::Number(5)],
            }
            .into(),
        );
        assert_eq!(
            expected_ast,
            lang_parser::statement("print(foo(5));").expect("Parser failed")
        )
    }

    // ===== Complex Expression Tests =====

    #[test]
    fn deeply_nested_arithmetic() {
        let expected_ast = AST::Add {
            left: AST::Multiply {
                left: AST::Subtract {
                    left: AST::Number(10).into(),
                    right: AST::Number(2).into(),
                }
                .into(),
                right: AST::Number(3).into(),
            }
            .into(),
            right: AST::Number(4).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("(10 - 2) * 3 + 4").expect("Parser failed")
        )
    }

    #[test]
    fn parentheses_override_precedence() {
        let expected_ast = AST::Multiply {
            left: AST::Number(2).into(),
            right: AST::Add {
                left: AST::Number(3).into(),
                right: AST::Number(4).into(),
            }
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("2 * (3 + 4)").expect("Parser failed")
        )
    }

    #[test]
    fn division_and_multiplication_precedence() {
        let expected_ast = AST::Divide {
            left: AST::Multiply {
                left: AST::Number(8).into(),
                right: AST::Number(4).into(),
            }
            .into(),
            right: AST::Number(2).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("8 * 4 / 2").expect("Parser failed")
        )
    }

    #[test]
    fn complex_boolean_expression() {
        let expected_ast = AST::Equal {
            left: AST::LessThan {
                left: AST::Number(5).into(),
                right: AST::Number(10).into(),
            }
            .into(),
            right: AST::Boolean(true).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("(5 < 10) == true").expect("Parser failed")
        )
    }

    #[test]
    fn not_with_comparison() {
        let expected_ast = AST::Not(
            AST::GreaterThan {
                left: AST::Number(3).into(),
                right: AST::Number(5).into(),
            }
            .into(),
        );
        assert_eq!(
            expected_ast,
            lang_parser::expression("!(3 > 5)").expect("Parser failed")
        )
    }

    #[test]
    fn chained_comparisons() {
        let expected_ast = AST::NotEqual {
            left: AST::GreaterThanEqual {
                left: AST::Id("x".to_string()).into(),
                right: AST::Number(10).into(),
            }
            .into(),
            right: AST::Boolean(false).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("(x >= 10) != false").expect("Parser failed")
        )
    }

    // ===== Number Parsing Tests =====

    #[test]
    fn large_number() {
        let expected_ast = AST::Number(999999);
        assert_eq!(
            expected_ast,
            lang_parser::expression("999999").expect("Parser failed")
        )
    }

    #[test]
    fn zero() {
        let expected_ast = AST::Number(0);
        assert_eq!(
            expected_ast,
            lang_parser::expression("0").expect("Parser failed")
        )
    }

    #[test]
    fn single_digit() {
        let expected_ast = AST::Number(7);
        assert_eq!(
            expected_ast,
            lang_parser::expression("7").expect("Parser failed")
        )
    }

    // ===== Boolean and Special Values Tests =====

    #[test]
    fn true_literal() {
        let expected_ast = AST::Boolean(true);
        assert_eq!(
            expected_ast,
            lang_parser::expression("true").expect("Parser failed")
        )
    }

    #[test]
    fn false_literal() {
        let expected_ast = AST::Boolean(false);
        assert_eq!(
            expected_ast,
            lang_parser::expression("false").expect("Parser failed")
        )
    }

    #[test]
    fn null_literal() {
        let expected_ast = AST::Null;
        assert_eq!(
            expected_ast,
            lang_parser::expression("null").expect("Parser failed")
        )
    }

    #[test]
    fn undefined_literal() {
        let expected_ast = AST::Undefined;
        assert_eq!(
            expected_ast,
            lang_parser::expression("undefined").expect("Parser failed")
        )
    }

    #[test]
    fn boolean_in_arithmetic() {
        let expected_ast = AST::Add {
            left: AST::Boolean(true).into(),
            right: AST::Number(1).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("true + 1").expect("Parser failed")
        )
    }

    // ===== Variable and Assignment Tests =====

    #[test]
    fn var_with_complex_expression() {
        let expected_ast = AST::Var {
            name: "result".to_string(),
            value: AST::Multiply {
                left: AST::Add {
                    left: AST::Number(5).into(),
                    right: AST::Number(3).into(),
                }
                .into(),
                right: AST::Number(2).into(),
            }
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("var result = (5 + 3) * 2;").expect("Parser failed")
        )
    }

    #[test]
    fn assign_array_element() {
        let expected_ast = AST::Assign {
            name: "x".to_string(),
            value: AST::ArrayLookup {
                array: AST::Id("arr".to_string()).into(),
                index: AST::Number(5).into(),
            }
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("x = arr[5];").expect("Parser failed")
        )
    }

    #[test]
    fn var_with_function_call() {
        let expected_ast = AST::Var {
            name: "len".to_string(),
            value: AST::ArrayLength(AST::Id("myArray".to_string()).into()).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("var len = length(myArray);").expect("Parser failed")
        )
    }

    // ===== Array Tests =====

    #[test]
    fn empty_array_literal() {
        let expected_ast = AST::ArrayLiteral(vec![]);
        assert_eq!(
            expected_ast,
            lang_parser::expression("[]").expect("Parser failed")
        )
    }

    #[test]
    fn array_with_mixed_types() {
        let expected_ast = AST::ArrayLiteral(vec![
            AST::Number(1),
            AST::Boolean(true),
            AST::Id("x".to_string()),
            AST::Null,
        ]);
        assert_eq!(
            expected_ast,
            lang_parser::expression("[1, true, x, null]").expect("Parser failed")
        )
    }

    #[test]
    fn array_with_expressions() {
        let expected_ast = AST::ArrayLiteral(vec![
            AST::Add {
                left: AST::Number(1).into(),
                right: AST::Number(2).into(),
            },
            AST::Multiply {
                left: AST::Number(3).into(),
                right: AST::Number(4).into(),
            },
        ]);
        assert_eq!(
            expected_ast,
            lang_parser::expression("[1 + 2, 3 * 4]").expect("Parser failed")
        )
    }

    #[test]
    fn nested_array_access() {
        let expected_ast = AST::ArrayLookup {
            array: AST::Id("matrix".to_string()).into(),
            index: AST::Add {
                left: AST::Id("i".to_string()).into(),
                right: AST::Number(1).into(),
            }
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("matrix[i + 1]").expect("Parser failed")
        )
    }

    // ===== Function Tests =====

    #[test]
    fn function_with_single_param() {
        let expected_ast = AST::Function {
            name: "double".to_string(),
            parameters: vec!["n".to_string()],
            body: AST::Block(vec![AST::Return {
                term: AST::Multiply {
                    left: AST::Id("n".to_string()).into(),
                    right: AST::Number(2).into(),
                }
                .into(),
            }])
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("function double(n) { return n * 2; }")
                .expect("Parser failed")
        )
    }

    #[test]
    fn function_with_four_params() {
        let expected_ast = AST::Function {
            name: "sum4".to_string(),
            parameters: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
            body: AST::Block(vec![AST::Return {
                term: AST::Add {
                    left: AST::Add {
                        left: AST::Add {
                            left: AST::Id("a".to_string()).into(),
                            right: AST::Id("b".to_string()).into(),
                        }
                        .into(),
                        right: AST::Id("c".to_string()).into(),
                    }
                    .into(),
                    right: AST::Id("d".to_string()).into(),
                }
                .into(),
            }])
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("function sum4(a, b, c, d) { return a + b + c + d; }")
                .expect("Parser failed")
        )
    }

    #[test]
    fn function_with_no_params() {
        let expected_ast = AST::Function {
            name: "getConstant".to_string(),
            parameters: vec![],
            body: AST::Block(vec![AST::Return {
                term: AST::Number(42).into(),
            }])
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("function getConstant() { return 42; }").expect("Parser failed")
        )
    }

    #[test]
    fn function_call_with_multiple_args() {
        let expected_ast = AST::Call {
            callee: "calculate".to_string(),
            args: vec![
                AST::Number(1),
                AST::Number(2),
                AST::Id("x".to_string()),
                AST::Add {
                    left: AST::Id("y".to_string()).into(),
                    right: AST::Number(5).into(),
                },
            ],
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("calculate(1, 2, x, y + 5)").expect("Parser failed")
        )
    }

    #[test]
    fn nested_function_calls() {
        let expected_ast = AST::Call {
            callee: "outer".to_string(),
            args: vec![AST::Call {
                callee: "inner".to_string(),
                args: vec![AST::Number(5)],
            }],
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("outer(inner(5))").expect("Parser failed")
        )
    }

    // ===== Control Flow Tests =====

    #[test]
    fn if_with_empty_else() {
        let expected_ast = AST::IfNode {
            conditional: AST::Id("x".to_string()).into(),
            consequence: AST::Block(vec![AST::Assign {
                name: "y".to_string(),
                value: AST::Number(1).into(),
            }])
            .into(),
            alternative: AST::Block(vec![]).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("if (x) { y = 1; } else { }").expect("Parser failed")
        )
    }

    #[test]
    fn while_with_multiple_statements() {
        let expected_ast = AST::While {
            conditional: AST::LessThan {
                left: AST::Id("i".to_string()).into(),
                right: AST::Number(10).into(),
            }
            .into(),
            body: AST::Block(vec![
                AST::Assign {
                    name: "sum".to_string(),
                    value: AST::Add {
                        left: AST::Id("sum".to_string()).into(),
                        right: AST::Id("i".to_string()).into(),
                    }
                    .into(),
                },
                AST::Assign {
                    name: "i".to_string(),
                    value: AST::Add {
                        left: AST::Id("i".to_string()).into(),
                        right: AST::Number(1).into(),
                    }
                    .into(),
                },
            ])
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("while (i < 10) { sum = sum + i; i = i + 1; }")
                .expect("Parser failed")
        )
    }

    #[test]
    fn return_with_complex_expression() {
        let expected_ast = AST::Return {
            term: AST::Multiply {
                left: AST::Call {
                    callee: "factorial".to_string(),
                    args: vec![AST::Subtract {
                        left: AST::Id("n".to_string()).into(),
                        right: AST::Number(1).into(),
                    }],
                }
                .into(),
                right: AST::Id("n".to_string()).into(),
            }
            .into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("return factorial(n - 1) * n;").expect("Parser failed")
        )
    }

    // ===== Block and Main Tests =====

    #[test]
    fn block_with_mixed_statements() {
        let expected_ast = AST::Block(vec![
            AST::Var {
                name: "x".to_string(),
                value: AST::Number(5).into(),
            },
            AST::Print(AST::Id("x".to_string()).into()),
            AST::Assign {
                name: "x".to_string(),
                value: AST::Number(10).into(),
            },
            AST::Assert(AST::Equal {
                left: AST::Id("x".to_string()).into(),
                right: AST::Number(10).into(),
            }.into()),
        ]);
        assert_eq!(
            expected_ast,
            lang_parser::statement("{ var x = 5; print(x); x = 10; assert(x == 10); }")
                .expect("Parser failed")
        )
    }

    #[test]
    fn main_with_all_statement_types() {
        let result = lang_parser::parser(
            r#"
            function main() {
                var x = 10;
                x = x + 1;
                if (x > 5) {
                    print(x);
                } else {
                    print(0);
                }
                while (x > 0) {
                    x = x - 1;
                }
                assert(x == 0);
                return 0;
            }
            "#,
        );
        assert!(result.is_ok(), "Parser should successfully parse main function with all statement types");
        if let Ok(AST::Main(statements)) = result {
            assert_eq!(statements.len(), 1, "Should have one function");
        }
    }

    // ===== Whitespace and Formatting Tests =====

    #[test]
    fn expression_with_no_spaces() {
        let expected_ast = AST::Add {
            left: AST::Multiply {
                left: AST::Number(2).into(),
                right: AST::Number(3).into(),
            }
            .into(),
            right: AST::Number(4).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("2*3+4").expect("Parser failed")
        )
    }

    #[test]
    fn expression_with_excessive_spaces() {
        let expected_ast = AST::Add {
            left: AST::Number(1).into(),
            right: AST::Number(2).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::expression("  1   +   2  ").expect("Parser failed")
        )
    }

    #[test]
    fn statement_with_newlines() {
        let expected_ast = AST::Var {
            name: "x".to_string(),
            value: AST::Number(42).into(),
        };
        assert_eq!(
            expected_ast,
            lang_parser::statement("var x\n=\n42;").expect("Parser failed")
        )
    }

    #[test]
    fn function_with_multiline_body() {
        let result = lang_parser::statement(
            "function test() {\n  var x = 1;\n  return x;\n}",
        );
        assert!(result.is_ok(), "Parser should handle multiline function bodies");
    }

    // ===== Edge Cases and Complex Scenarios =====

    #[test]
    fn deeply_nested_blocks() {
        let result = lang_parser::statement(
            "{ { { var x = 1; } } }",
        );
        assert!(result.is_ok(), "Parser should handle deeply nested blocks");
    }

    #[test]
    fn long_identifier_name() {
        let expected_ast = AST::Id("thisIsAVeryLongVariableNameForTesting".to_string());
        assert_eq!(
            expected_ast,
            lang_parser::expression("thisIsAVeryLongVariableNameForTesting")
                .expect("Parser failed")
        )
    }

    #[test]
    fn identifier_with_numbers() {
        let expected_ast = AST::Id("var123test".to_string());
        assert_eq!(
            expected_ast,
            lang_parser::expression("var123test").expect("Parser failed")
        )
    }

    #[test]
    fn identifier_with_underscores() {
        let expected_ast = AST::Id("my_var_name".to_string());
        assert_eq!(
            expected_ast,
            lang_parser::expression("my_var_name").expect("Parser failed")
        )
    }

    #[test]
    fn comparison_all_operators() {
        // Test all comparison operators parse correctly
        assert!(lang_parser::expression("a == b").is_ok());
        assert!(lang_parser::expression("a != b").is_ok());
        assert!(lang_parser::expression("a < b").is_ok());
        assert!(lang_parser::expression("a > b").is_ok());
        assert!(lang_parser::expression("a <= b").is_ok());
        assert!(lang_parser::expression("a >= b").is_ok());
    }

    #[test]
    fn all_arithmetic_operators() {
        // Test all arithmetic operators parse correctly
        assert!(lang_parser::expression("a + b").is_ok());
        assert!(lang_parser::expression("a - b").is_ok());
        assert!(lang_parser::expression("a * b").is_ok());
        assert!(lang_parser::expression("a / b").is_ok());
    }

    #[test]
    fn assert_with_complex_condition() {
        let expected_ast = AST::Assert(
            AST::Equal {
                left: AST::Call {
                    callee: "factorial".to_string(),
                    args: vec![AST::Number(5)],
                }
                .into(),
                right: AST::Number(120).into(),
            }
            .into(),
        );
        assert_eq!(
            expected_ast,
            lang_parser::statement("assert(factorial(5) == 120);").expect("Parser failed")
        )
    }

    #[test]
    fn multiple_functions_in_program() {
        let result = lang_parser::parser(
            r#"
            function helper(x) { return x * 2; }
            function main() { return helper(5); }
            "#,
        );
        assert!(result.is_ok(), "Parser should handle multiple functions");
        if let Ok(AST::Main(statements)) = result {
            assert_eq!(statements.len(), 2, "Should have two functions");
        }
    }
}
