use crate::ast::AST;
use crate::ast2::ASTStruct;

peg::parser! {
  grammar lang_parser() for str {

    rule _ = [' ' | '\n']*

    pub rule Number() -> AST
      = n:$(['0'..='9']+) { AST::Number(n.parse().unwrap()) }

    pub rule Id() -> AST
      = n:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | '_' |  'A'..='Z' | '0'..='9' ]+) { AST::Id(n.to_string()) }


    pub rule args() -> Vec<AST>
      = expression() ** ","

    ///
    ///
    pub rule call() -> AST
      = n:Id() _ "("  _ a:args() _ ")" {
      AST::Call {
        callee: n.to_string(),
        args:a.iter().cloned().map(|e| Box::new(e)).collect()
      }
    }

    pub rule unary() -> AST
      = n:"!"? a:atom() {
        match n {
          Some(term) => AST::Not(Box::new(a)),
          None => a
      }
    }


   rule sum() -> AST
        = l:product() _ op:$("+" / "-") _ r:product() {
             let ast = match op {
                "+" =>  AST::Add{left:Box::new(l), right:Box::new(r)},
                "-" =>  AST::Subtract{left:Box::new(l), right:Box::new(r)},
                 x => panic!("sum found op {}", x)
            };
            ast
        }
        / product()

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

            } / atom()

    pub rule atom() -> AST
      = call() / Id() / Number() / "("  e:sum() ")" { e }


      pub rule expression() -> AST
            = sum()



    ///
    /// keywords
    ///
    pub rule FUNCTION() -> AST
      = "function"  { AST::Token("function".to_string()) }

    pub rule IF() -> AST
      = "if"  { AST::Token("if".to_string()) }

    pub rule ELSE() -> AST
      = "else"  { AST::Token("else".to_string()) }

    pub rule RETURN() -> AST
      = "return"  { AST::Token("return".to_string()) }

    pub rule VAR() -> AST
      = "var"  { AST::Token("var".to_string()) }

    pub rule WHILE() -> AST
      = "while"  { AST::Token("while".to_string()) }



  }
}


#[cfg(test)]
mod tests {
    use crate::ast;
    use super::*;

    #[test]
    fn number() {
        assert_eq!(AST::Number(1233), lang_parser::expression("1233").expect("Parser failed"));
    }

    #[test]
    fn id() {
        assert_eq!(AST::Id("varname".to_string()), lang_parser::expression("varname").expect("Parser failed"));
        assert_eq!(AST::Id("varname2".to_string()), lang_parser::expression("varname2").expect("Parser failed"));
        assert_eq!(AST::Id("var_name2".to_string()), lang_parser::expression("var_name2").expect("Parser failed"));
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
        assert_eq!(expected_ast, lang_parser::expression("1+3*2").expect("Parser failed"))
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

        assert_eq!(expected_ast, lang_parser::expression("(1+3) * 2").expect("Parser failed"));
    }
    #[test]
    fn call_simple() {
        let expected_ast = AST::Call {
            callee: "fname".to_string(),
            args: vec![]
        };
        println!("{}", expected_ast);
        assert_eq!(expected_ast, lang_parser::expression("fname()").expect("Parser failed"));
    }

    #[test]
    fn call_w_args_vars() {



        let expected_ast = AST::Call {
            callee: "fname".to_string(),
            args: vec![ Box::new(AST::Id("a".to_string())), Box::new(AST::Id("b".to_string()))]
        };
        println!("{}", expected_ast);
        assert_eq!(expected_ast, lang_parser::expression("fname (a,b)").expect("Parser failed"));

    }

    #[test]
    fn call_w_args() {
        let expected_ast = AST::Call {
            callee: "myFunction".to_string(),
            args: vec![
                Box::new(AST::Add {
                    left: Box::new(AST::Number(1)),
                    right: Box::new(AST::Number(1)),
                }),
                Box::new(AST::Id("a".to_string())),
            ],
        };
        println!("{}", expected_ast);
        assert_eq!(expected_ast, lang_parser::expression("myFunction(1+1,a)").expect("Parser failed"));
    }
}
