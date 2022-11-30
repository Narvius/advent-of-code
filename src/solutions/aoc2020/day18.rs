/// Calculate the sum of all expressions when evaluated with equal precedence.
pub fn one(input: &str) -> crate::Result<i64> {
    Ok(input.lines().filter_map(|line| eval(line, false)).sum())
}

/// Calculate the sum of all expressions when evaluated with higher-precedence addition.
pub fn two(input: &str) -> crate::Result<i64> {
    Ok(input.lines().filter_map(|line| eval(line, true)).sum())
}

/// Calculates the value of an expression. If `addition_precedence` is true, addition has
/// higher precedence than multiplication, otherwise they have the same precedence.
///
/// Uses the [shunting-yard algorithm][1] to convert the expression to RPN before evaluating it.
///
/// [1]: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
fn eval(expr: &str, addition_precedence: bool) -> Option<i64> {
    // Returns whether `a` has less precedence than `b`.
    fn prec(a: &str, b: &str, add: bool) -> bool {
        let add: i32 = add.into();
        let a = if a == "+" { 1 + add } else { 1 };
        let b = if b == "+" { 1 + add } else { 1 };
        a < b
    }

    let mut rpn = vec![];

    // Convert the expression to RPN.
    let mut stack = vec![];
    let expr = expr.replace('(', "( ").replace(')', " )");
    for token in expr.split_ascii_whitespace() {
        match token {
            token if token.chars().all(|c| c.is_ascii_digit()) => rpn.push(token),
            "+" | "*" => {
                while !stack.is_empty() {
                    let item = *stack.last()?;
                    if item == "(" || prec(item, token, addition_precedence) {
                        break;
                    }
                    rpn.push(stack.pop()?);
                }
                stack.push(token);
            }
            "(" => stack.push(token),
            ")" => {
                while let Some(t) = stack.pop() {
                    if t == "(" {
                        break;
                    }
                    rpn.push(t);
                }
            }
            _ => {}
        }
    }

    while !stack.is_empty() {
        rpn.push(stack.pop()?);
    }

    // Evaluate.
    let mut stack = vec![];
    for token in rpn {
        if let Ok(n) = token.parse() {
            stack.push(n)
        } else {
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(match token {
                "+" => a + b,
                "*" => a * b,
                _ => return None,
            })
        }
    }
    stack.pop()
}
