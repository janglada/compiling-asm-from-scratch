use crate::ast::AST;
use crate::error::CompileError;

peg::parser! {
  pub grammar lang_parser() for str {

    rule _ = quiet!{[' ' | '\n' | '\t']*}


    pub rule Number() -> AST
      = n:$(['0'..='9']+) { AST::Number(n.parse().unwrap()) }

    pub rule Id() -> AST
      = n:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | '_' |  'A'..='Z' | '0'..='9' ]*) { AST::Id(n.to_string()) }


    pub rule args() -> Vec<AST>
      = expression() ** (_ "," _)


    ///
    ///
    pub rule call() -> AST
      = callee:Id() _ "(" _ a:args() _ ")" {
            if callee.to_string() == "assert" {
                let mut iter = a.into_iter().take(1);
                let ast: AST = iter.next().unwrap();
                AST::Assert(Box::new(ast))
            } else {
              AST::Call {
                callee: callee.to_string(),
                args:a.to_vec()
              }
            }
    }
   ///
   /// INFIX
   // pub rule sum() -> AST
   //      = _ l:product() _ op:$("+" / "-") _ r:product()  _ {
   //
   //          match op {
   //              "+" =>  AST::Add{left:l.into(), right:r.into()},
   //              "-" =>  AST::Subtract{left:l.into(), right:r.into()},
   //               x => panic!("sum found op {}", x)
   //          }
   //      } / product()
   //
   // pub rule product() -> AST
   //      = _ l:comparison() _ op:$("*" / "/") _ r:comparison() _ {
   //
   //          match op {
   //              "*" =>  AST::Multiply{left:l.into(), right:r.into()},
   //              "/" =>  AST::Divide{left:l.into(), right:r.into()},
   //               x => panic!("product found op {}", x)
   //          }
   //
   //      }
   //      / comparison()
   //
   //
   //  pub  rule comparison() -> AST
   //          = _ l:unary() _ op:$("==" / "!=") _ r:unary() _ {
   //
   //              match op {
   //                  "==" =>  AST::Equal{left: l.into(), right: r.into()},
   //                  "!=" =>  AST::NotEqual{left:l.into(), right:r.into()},
   //              x => panic!("comparison found op {}", x)
   //              }
   //
   //          } / unary()
   //
   //  pub rule unary() -> AST
   //    = n:("!")? a:atom() {
   //      match n {
   //        Some(term) => AST::Not(a.into()),
   //        None => a
   //    }
   //  }
    pub rule atom() -> AST
      = call() / Id() / Number()

    /// alow whitespaces before after
    pub rule expression() -> AST = _ e:expressionPrecedence() _ {e }

    pub rule expressionPrecedence() -> AST = precedence!{
        x:(@) _ "==" _ y:@ { AST::Equal{left:x.into(), right:y.into()} }
        x:(@) _ "!=" _ y:@ { AST::NotEqual{left:x.into(), right:y.into()} }
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
            = "while"  _ "(" _ conditional: expression()  _ ")"  _   body: statement() _ {
            AST::While {
                conditional: conditional.into(),
                body: body.into()
            }
        }

  pub rule varStmt() -> AST
            = VAR()  _ id:Id() _ ASSIGN() _ value:expression() _ ";" _{
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
            =  id:Id() _ ASSIGN() _ value:expression() _ ";" _ {
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

                if false && name == "main" {
                  if let AST::Block(statements) = body {
                    AST::Main(statements)
                  } else {
                        unreachable!()
                   }
                } else {

                    AST::Function {
                        name,
                        parameters: p,
                        body: body.into()
                    }
                }

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
    pub rule ASSIGN() = "="

    pub rule FUNCTION() = "function"

    pub rule IF() = "if"

    pub rule ELSE() = "else"

    pub rule RETURN() = "return"

    pub rule VAR() = "var"

    pub rule WHILE() = "while"



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
        let expected_ast = AST::Main(vec![AST::Block(vec![
            AST::Assert(1.into()),
            AST::Assert(1.into()),
            AST::Call {
                callee: "putchar".into(),
                args: vec![AST::Number(84)],
            },
        ])]);
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
        let expected_ast = AST::Main(vec![AST::Block(vec![
            AST::IfNode {
                conditional: 0.into(),
                consequence: AST::Block(vec![AST::Assert(0.into())]).into(),
                alternative: AST::Block(vec![AST::Assert(1.into())]).into(),
            },
            AST::Call {
                callee: "putchar".into(),
                args: vec![AST::Number(84)],
            },
        ])]);
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
        let expected_ast = AST::Main(vec![
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
        ]);

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
            AST::Main(vec![AST::Var {
                name: "a".into(),
                value: 1.into(),
            }]),
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
}
