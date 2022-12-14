
use crate::ast::AST;
use crate::ast2::ASTStruct;

peg::parser! {
  grammar lang_parser() for str {
    pub rule Number() -> AST
      = n:$(['0'..='9']+) { AST::Number(n.parse().unwrap()) }

    pub rule Id() -> AST
      = n:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9' ]+) { AST::Id(n.to_string()) }


    pub rule args() -> Vec<AST>
      = l:(expression() ** ",") { l }

    pub rule call() -> AST
      = name:(Id()) l:LEFT_PAREN a:args() r:RIGHT_PAREN {
      AST::Call {
        callee: name.to_string(),
        args:a
      }
    }

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

    ///
    /// Token
    ///
    pub rule NOT()
      = n: "!"
    pub rule EQUAL()
      = n: "=="
    pub rule NOT_EQUAL()
      = n: "!="
    pub rule PLUS()
      = n: "+"
    pub rule MINUS()
      = n: "-"
    pub rule STAR()
      = n: "*"
    pub rule SLASH()
      = n: "/"
    ///
    /// punctuation
    ///

    pub rule COMMA()
      = n: ","

     pub rule SEMICOLON()
      = ";"

    pub rule LEFT_PAREN()
      = "("
    pub rule RIGHT_PAREN()
      = ")"
    pub rule LEFT_BRACE()
      = "{"
    pub rule RIGHT_BRACE()
      = "}"
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
  use crate::ast2::Number;
  use super::*;

  #[test]
  fn number() {



    if let AST::Number(n) = parseNumber("1233") {
      assert_eq!(1233, n)
    } else {
      panic!()
    }

  }

  #[test]
  fn id() {

    if let AST::Id(n) = parseId("varname") {
      assert_eq!("varname", n)
    } else {
      panic!()
    }

    if let AST::Id(n) = parseId("varname2") {
      assert_eq!("varname2", n)
    } else {
      panic!()
    }

  }

}
