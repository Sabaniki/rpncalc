use anyhow::{Result, Context};

pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(n) = token.parse::<i32>() {
                stack.push(n);
            } else {
                let y = stack.pop().context("invalid syntax at {}", pos)?;
                let x = stack.pop().context("invalid syntax {}", pos)?;

                let result = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bali!("invalid token at {}", pos),
                };
                stack.push(result);
            }

            // `-v` オプションが指定されている場合は，この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}