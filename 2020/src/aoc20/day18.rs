
#[allow(unused)]
pub fn input() -> Vec<String> {
    crate::aoc::input(20, 18)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Mul
}
impl Operator {
    pub fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Num(i64),
    Op(Operator),
    ParOpen,
    ParClose,
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Terminal(i64),
    Expression(Box<Self>, Operator, Box<Self>),
}

fn tokenize(inputs: &str) -> Vec<Token> {
    let mut number = String::new();
    let mut tokens = Vec::new();
    for c in inputs.chars() {
        match c {
            '+'|'*'|'('|')' => {
                if number.len() > 0 {
                    tokens.push(Token::Num(number.parse::<i64>().unwrap()));
                    number = String::new();
                }
                match c {
                    '+' => tokens.push(Token::Op(Operator::Add)),
                    '*' => tokens.push(Token::Op(Operator::Mul)),
                    '(' => tokens.push(Token::ParOpen),
                    ')' => tokens.push(Token::ParClose),
                    _ => unreachable!(),
                }
            },
            ' ' => (),
            _ => number.push(c)
        }
    }
    if number.len() > 0 {
        tokens.push(Token::Num(number.parse::<i64>().unwrap()));
    }
    tokens
}


fn build_expression(tokens: &[Token]) -> Expression {
    use Token::*;
    let mut call_stack = Vec::new();
    let mut expr_stack = None; // Nesting stack
    let mut oper_stack = None;
    for token in tokens {
        match token {
            Op(x) => oper_stack = Some(Op(*x)),
            Num(x) => {
                if let Some(Op(y)) = oper_stack {
                    let expr = Expression::Expression(
                            Box::new(expr_stack.unwrap()),
                            y,
                            Box::new(Expression::Terminal(*x)),
                    );
                    oper_stack = None;
                    expr_stack = Some(expr);
                } else {
                    expr_stack = Some(Expression::Terminal(*x))
                }
            },
            ParOpen => {
                // expr_stack is 1 expr
                // oper_stack is 1 operator
                // or both are None, if parenthesis appears on beginning
                call_stack.push((expr_stack.clone(), oper_stack.clone()));
                expr_stack = None;
                oper_stack = None;
            },
            ParClose => {
                // expr_stack is 1 expr
                // oper_stack is empty
                let (expr1, op) = call_stack.pop().unwrap();
                if let Some(Op(x)) = op {
                    expr_stack = Some(
                        Expression::Expression(
                            Box::new(expr1.unwrap()),
                            x,
                            Box::new(expr_stack.unwrap())
                        )
                    )
                }
            }
        }
    }
    expr_stack.unwrap()
}

fn apply_precedence(tokens: &[Token], priority: &[Operator]) -> Vec<Token> {
    use Token::*;
    let mut prev = Vec::from(tokens);
    let mut ret = Vec::new();

    for operator in priority {
        ret = vec![ParOpen];
        for t in prev {
            match t {
                Op(x) => {
                    if x != *operator {
                        ret.push(ParClose);
                        ret.push(Op(x));
                        ret.push(ParOpen);
                    } else {
                        ret.push(Op(x));
                    }
                },
                Num(x) => {
                    ret.push(Num(x))
                },
                ParOpen => {
                    ret.push(ParOpen);
                    ret.push(ParOpen);
                },
                ParClose => {
                    ret.push(ParClose);
                    ret.push(ParClose);
                }
            }
        }
        ret.push(ParClose);
        prev = ret.clone();
    }
    ret
}


fn evaluate_expression(expression: Expression) -> i64 {
    match expression {
        Expression::Terminal(x) => x,
        Expression::Expression(a, op, b) => op.apply(evaluate_expression(*a), evaluate_expression(*b)),
    }
}

#[allow(unused)]
pub fn part1(inputs: &[String]) -> i64 {
    inputs.iter()
    .map(|line| 
        evaluate_expression(
            build_expression(&tokenize(line))
        )
    )
    .sum()
}

#[allow(unused)]
pub fn part2(inputs: &[String]) -> i64 {
    inputs.iter()
    .map(|line| 
        evaluate_expression(
            build_expression(
                &apply_precedence(&tokenize(line), &[Operator::Add])
            )
        )
    )
    .sum()
}


#[test]
fn test_day18_tokenize() {
    assert_eq!(vec![Token::Num(1), Token::Op(Operator::Add), Token::Num(12)], tokenize("1 + 12"));
    assert_eq!(tokenize("10 * (3 + 4)"), vec![
        Token::Num(10), Token::Op(Operator::Mul), Token::ParOpen, 
        Token::Num(3), Token::Op(Operator::Add), Token::Num(4), Token::ParClose]);
}

#[test]
fn test_day18_build_expression() {
    let expr = build_expression(&vec![Token::Num(1), Token::Op(Operator::Add), Token::Num(12)]);
    assert_eq!(expr, Expression::Expression(Box::new(Expression::Terminal(1)), Operator::Add, Box::new(Expression::Terminal(12))));

    let expr = build_expression(&vec![Token::ParOpen, Token::Num(1), Token::Op(Operator::Add),
        Token::Num(2), Token::ParClose, Token::Op(Operator::Mul), 
        Token::ParOpen, Token::Num(4), Token::ParClose]);
    assert_eq!(expr, Expression::Expression(
        Box::new(Expression::Expression(Box::new(Expression::Terminal(1)), Operator::Add, Box::new(Expression::Terminal(2)))),
        Operator::Mul,
        Box::new(Expression::Terminal(4))
    ))
}

#[test]
fn test_day18_evaluate_expression() {
    let ans = evaluate_expression(Expression::Expression(Box::new(Expression::Terminal(1)), Operator::Add, Box::new(Expression::Terminal(12))));
    assert_eq!(13, ans);

    let ans = evaluate_expression(Expression::Expression(
        Box::new(Expression::Expression(Box::new(Expression::Terminal(1)), Operator::Add, Box::new(Expression::Terminal(2)))),
        Operator::Mul,
        Box::new(Expression::Terminal(4))));
    assert_eq!(12, ans);
}

#[test]
fn test_apply_precedence() {
    let tokens = apply_precedence(&tokenize("1 + 2 * 4"), &[Operator::Add]);
    assert_eq!(tokens, vec![Token::ParOpen, Token::Num(1), Token::Op(Operator::Add),
    Token::Num(2), Token::ParClose, Token::Op(Operator::Mul), 
    Token::ParOpen, Token::Num(4), Token::ParClose]);

    assert_eq!(apply_precedence(&tokenize("1 + 2 * 4"), &[Operator::Add]), tokenize("(1 + 2) * (4)"));
    assert_eq!(apply_precedence(&tokenize("1 + (2 * 3) + (4 * 5)"), &[Operator::Add]), tokenize("(1 + ((2) * (3)) + ((4) * (5)))"));

}

#[test]
fn test_day18_token_build_eval() {
    let inputs = vec![
        "2 * 3 + (4 * 5)", 
        "5 + (8 * 3 + 9 + 3 * 4 * 3)", 
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"];
    let results = [26, 437, 12240, 13632];
    for (i, line) in inputs.iter().enumerate() {
        assert_eq!(results[i], evaluate_expression(build_expression(&tokenize(line))));
    }
}

#[test]
fn test_day18_token_build2_eval() {
    let inputs = vec![
        "1 + (2 * 3) + (4 * (5 + 6))", 
        "2 * 3 + (4 * 5)",
        "5 + (8 * 3 + 9 + 3 * 4 * 3)", 
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"];
    let results = [51, 46, 1445, 669060, 23340];
    for (i, line) in inputs.iter().enumerate() {
        assert_eq!(results[i], evaluate_expression(
            build_expression(
                &apply_precedence(&tokenize(line), &[Operator::Add])
            )
        ));
    }
}

#[test]
fn test_day18_part1() {
    let inputs = vec![
        String::from("2 * 3 + (4 * 5)"), 
        String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 
        String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")];
    assert_eq!(26335, part1(&inputs));
}

#[test]
fn test_day18_part2() {
    let inputs = vec![
        String::from("1 + (2 * 3) + (4 * (5 + 6))"), 
        String::from("2 * 3 + (4 * 5)"),
        String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 
        String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")];
    assert_eq!([51, 46, 1445, 669060, 23340].iter().sum::<i64>(), part2(&inputs));
}


#[test]
fn run_day18() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 13 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 13 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}