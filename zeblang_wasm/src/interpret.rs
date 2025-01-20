use zeblang::{ExpressionNode, StatementNode};

struct Interpreter {}

pub fn interpret(parse_tree: Vec<StatementNode>) -> Result<i32, String> {
    return Interpreter {}.run(parse_tree);
}

impl Interpreter {
    fn run(&mut self, parse_tree: Vec<StatementNode>) -> Result<i32, String> {
        let mut out = Err("Error".to_string());
        for statement in parse_tree.into_iter() {
            match statement {
                StatementNode::Exit(node) => {
                    out = self.interpret_exit(node);
                    break;
                }
                StatementNode::Assign(_, _) => todo!(),
                _ => todo!(),
            }
        }
        return out;
    }

    fn interpret_exit(&mut self, node: ExpressionNode) -> Result<i32, String> {
        let value = self.interpret_expr(node);
        return value;
    }

    fn interpret_expr(&mut self, node: ExpressionNode) -> Result<i32, String> {
        return match node {
            ExpressionNode::Value(value) => {
                value.parse::<i32>().ok().ok_or("invalid int".to_string())
            }
            ExpressionNode::Infix(node1, infix, node2) => {
                let v1 = self.interpret_expr(*node1)?;
                let v2 = self.interpret_expr(*node2)?;
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
