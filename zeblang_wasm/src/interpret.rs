use std::collections::HashMap;
use std::slice::Iter;

use zeblang::{ExpressionNode, StatementNode};

struct Interpreter<'a> {
    vars: HashMap<String, i32>,
    iter: Iter<'a, StatementNode>,
    out: Result<i32, String>,
}

pub fn interpret(parse_tree: Vec<StatementNode>) -> Result<i32, String> {
    return Interpreter {
        vars: HashMap::new(),
        iter: parse_tree.iter(),
        out: Err("Error".to_string()),
    }
    .run();
}

// need to make a parse statement function we can call recursively
// do we need to first translate everything into executable units that we then call in order
// i think maybe so
impl<'a> Interpreter<'a> {
    fn run(&mut self) -> Result<i32, String> {
        let mut statement_option = self.iter.next();
        while statement_option.is_some() {
            let statement = statement_option.ok_or("Some Looping went wrong!")?;
            self.interpret_statement(statement)?;
            statement_option = self.iter.next();
        }
        return self.out.to_owned();
    }

    fn interpret_statement(&mut self, statement: &StatementNode) -> Result<(), String> {
        match statement {
            StatementNode::Exit(node) => {
                self.out = self.interpret_exit(node);
                return Ok(());
            }
            StatementNode::Assign(name, node) => {
                let value = self.interpret_expr(node)?;
                self.vars.insert(name.to_owned(), value);
            }
            StatementNode::While(node) => {
                let mut next = self.iter.next().ok_or("While loop not closed")?;
                let mut stmnts: Vec<&StatementNode> = Vec::new();
                while next != &StatementNode::EndWhile {
                    stmnts.push(next);
                    next = self.iter.next().ok_or("While loop not closed")?;
                }
                while self.interpret_expr(node)? != 0 {
                    for stmnt in stmnts.iter() {
                        self.interpret_statement(stmnt)?;
                    }
                }
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn interpret_exit(&mut self, node: &ExpressionNode) -> Result<i32, String> {
        let value = self.interpret_expr(node);
        return value;
    }

    fn interpret_expr(&mut self, node: &ExpressionNode) -> Result<i32, String> {
        return match node {
            ExpressionNode::Value(value) => {
                value.parse::<i32>().ok().ok_or("invalid int".to_string())
            }
            ExpressionNode::Var(name) => Ok(self.vars.get(name).ok_or("Undefined var")?.to_owned()),
            ExpressionNode::Infix(node1, infix, node2) => {
                let v1 = self.interpret_expr(node1)?;
                let v2 = self.interpret_expr(node2)?;
                match infix.as_str() {
                    "+" => Ok(v1 + v2),
                    "-" => Ok(v1 - v2),
                    "*" => Ok(v1 * v2),
                    "/" => Ok(v1 / v2),
                    "==" => Ok((v1 == v2) as i32),
                    "!=" => Ok((v1 != v2) as i32),
                    "%" => Ok(v1 % v2),
                    _ => Err("Invalid Infix op".to_string()),
                }
            }
            _ => todo!(),
        };
    }
}
