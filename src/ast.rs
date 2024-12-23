use std::io::Write;
use std::{fmt, writeln};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    Number(u64),
    Id(String),
    Not(Box<AST>),
    Equal {
        left: Box<AST>,
        right: Box<AST>,
    },
    NotEqual {
        left: Box<AST>,
        right: Box<AST>,
    },
    Add {
        left: Box<AST>,
        right: Box<AST>,
    },
    Subtract {
        left: Box<AST>,
        right: Box<AST>,
    },
    Multiply {
        left: Box<AST>,
        right: Box<AST>,
    },
    Divide {
        left: Box<AST>,
        right: Box<AST>,
    },

    LessThan {
        left: Box<AST>,
        right: Box<AST>,
    },

    GreaterThan {
        left: Box<AST>,
        right: Box<AST>,
    },

    LessThanEqual {
        left: Box<AST>,
        right: Box<AST>,
    },

    GreaterThanEqual {
        left: Box<AST>,
        right: Box<AST>,
    },

    Call {
        callee: String,
        args: Vec<AST>,
    },
    Return {
        term: Box<AST>,
    },
    Block(Vec<AST>),

    IfNode {
        conditional: Box<AST>,
        consequence: Box<AST>,
        alternative: Box<AST>,
    },
    Function {
        name: String,
        parameters: Vec<String>,
        body: Box<AST>,
    },
    Var {
        name: String,
        value: Box<AST>,
    },
    Assign {
        name: String,
        value: Box<AST>,
    },
    While {
        conditional: Box<AST>,
        body: Box<AST>,
    },
    Undefined,
    Null,

    Boolean(bool),
    ArrayLiteral(Vec<AST>),
    ArrayLookup {
        array: Box<AST>,
        index: Box<AST>,
    },
    ArrayLength(Box<AST>),

    Main(Vec<AST>),
    Assert(Box<AST>),
}

impl From<u8> for Box<AST> {
    fn from(val: u8) -> Self {
        Box::new(AST::Number(val as u64))
    }
}

impl From<String> for Box<AST> {
    fn from(val: String) -> Self {
        Box::new(AST::Id(val))
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            AST::Number(value) => write!(f, "{}", value),
            AST::Id(name) => write!(f, "{}", name),
            AST::Not(value) => write!(f, "!{}", value),
            AST::Equal { left, right } => write!(f, "({} == {})", left, right),
            AST::NotEqual { left, right } => write!(f, "({} != {})", left, right),
            AST::Add { left, right } => write!(f, "({} + {})", left, right),
            AST::Subtract { left, right } => write!(f, "({} - {})", left, right),
            AST::Multiply { left, right } => write!(f, "({} * {})", left, right),
            AST::Divide { left, right } => write!(f, "({} / {})", left, right),
            AST::LessThan { left, right } => write!(f, "({} < {})", left, right),
            AST::GreaterThan { left, right } => write!(f, "({} > {})", left, right),
            AST::LessThanEqual { left, right } => write!(f, "({} <= {})", left, right),
            AST::GreaterThanEqual { left, right } => write!(f, "({} >= {})", left, right),
            AST::Boolean(value) => write!(f, "{}", value),

            AST::Call { callee, args } => write!(
                f,
                "{} ({})",
                callee,
                args.iter().map(|x| x.to_string() + ",").collect::<String>()
            ),
            AST::Return { term } => write!(f, "return {}", term),
            AST::Block(statements) => {
                writeln!(f, "{{").unwrap();
                for stmt in statements {
                    writeln!(f, " {};", stmt).unwrap();
                }
                write!(f, "}}")
            }
            AST::IfNode {
                conditional,
                consequence,
                alternative,
            } => write!(
                f,
                "if ({})\n{}\n else\n{}",
                conditional, consequence, alternative
            ),
            AST::Function {
                name,
                parameters,
                body,
            } => write!(
                f,
                "function {}({})\n{{\n{}\n}}\n",
                name,
                parameters.join(","),
                body
            ),
            AST::Var { name, value } => write!(f, "var {} = {}", name, value),
            AST::Assign { name, value } => write!(f, "{} = {}", name, value),
            AST::While { conditional, body } => {
                write!(f, "while ({})\n{}", conditional, body)
            }
            AST::Main(statements) => {
                write!(f, "{:?}", statements)
            }
            AST::Assert(condition) => {
                write!(f, "assert({})", condition)
            }
            AST::Undefined => {
                write!(f, "undefined")
            }
            AST::Null => {
                write!(f, "null")
            }
            AST::ArrayLiteral(args) => {
                let mut vec_str = args.into_iter().map(|v| v.to_string()).collect::<Vec<_>>();

                write!(f, "[{}]", vec_str.join(","))
                // for v in args.iter() {
                //     write!(f, "{}", v.to_string());
                // }
                // write!(f, "]\n")
            }
            AST::ArrayLookup { array, index } => {
                write!(f, "array[{}]\n", index)
            }
            AST::ArrayLength(array) => {
                write!(f, "array.length\n")
            }
        }
    }
}

// impl Into<Box<AST>> for AST {
//     fn into(self) -> Box<AST> {
//         Box::new(self)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::ast::AST;

    #[test]
    fn check_equals() {
        let ast1 = AST::Add {
            left: Box::new(AST::Number(1)),
            right: Box::new(AST::Multiply {
                left: Box::new(AST::Number(3)),
                right: Box::new(AST::Number(3)),
            }),
        };
        let ast2 = AST::Add {
            left: Box::new(AST::Number(1)),
            right: Box::new(AST::Multiply {
                left: Box::new(AST::Number(3)),
                right: Box::new(AST::Number(3)),
            }),
        };

        assert_eq!(ast1, ast2);
    }

    #[test]
    fn add() {
        let ast = AST::Add {
            left: AST::Number(42).into(),
            right: AST::Not(
                AST::NotEqual {
                    left: AST::Number(20).into(),
                    right: AST::Number(20).into(),
                }
                .into(),
            )
            .into(),
        };

        println!("{}", ast);
    }
}
