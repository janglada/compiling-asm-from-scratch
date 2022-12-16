
use crate::ast::AST;
use crate::ast2::ASTStruct;

peg::parser! {
  grammar lang_parser() for str {

    rule _ = [' ' | '\n']*

    pub rule Number() -> AST
      = n:$(['0'..='9']+) { AST::Number(n.parse().unwrap()) }

    pub rule Id() -> AST
      = n:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9' ]+) { AST::Id(n.to_string()) }


    pub rule args() -> Vec<AST>
      = l:(expression() ** ",") { l }

    pub rule call() -> AST
      = n:Id() "(" a:args() ")" {
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
        = l:atom() _ op:$("*" / "/") _ r:atom() {
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



      pub rule expression() -> AST
            = comparison()

    pub rule atom() -> AST
      = call() / Id() / Number() / "("  e:expression() ")" { e }

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
pub fn parseNumber(input:&str) -> AST {
  lang_parser::Number(input).unwrap()
}
pub fn parseId(input:&str) -> AST {
  lang_parser::Id(input).unwrap()
}
#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::ast2::Number;
  use super::*;

  #[test]
  fn number() {

    if let Ok(AST::Number(n)) = lang_parser::expression("1233") {
      assert_eq!(1233, n)
    } else {
      panic!()
    }


  }

  #[test]
  fn id() {

    if let Ok(AST::Id(n)) = lang_parser::expression("varname") {
      assert_eq!("varname", n)
    } else {
      panic!()
    }

    if let Ok(AST::Id(n)) = lang_parser::expression("varname2") {
      assert_eq!("varname2", n)
    } else {
      panic!()
    }

  }

    #[test]
    fn arithmetic() {
        let expected_ast =  AST::Add{
            left:Box::new(AST::Number(1)), right: Box::new(AST::Number(1))
        };
        let run =  lang_parser::expression("1+1");

        println!("{:?}", run);
        if let Ok(expected_ast) = lang_parser::expression("1+1") {
          //  assert_eq!("varname", n)
        } else {
            panic!()
        }



    }


}
