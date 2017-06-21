#[macro_use]
extern crate nom;


use std::str::FromStr;

use nom::*;


#[derive(Debug)]
enum Operator {
    Addition,
    Division,
    Multiplication,
    Subtraction,
}

#[derive(Debug)]
struct Expression {
    lhs: i32,
    op: Operator,
    rhs: i32,
}

impl Expression {

    fn eval(&self) -> i32 {
        match &self.op {
            &Operator::Addition => self.lhs + self.rhs,
            &Operator::Division => self.lhs / self.rhs,
            &Operator::Multiplication => self.lhs * self.rhs,
            &Operator::Subtraction => self.lhs - self.rhs,
        }
    }

}

named!(parse_number<&str, i32>,
    do_parse!(
        sign: opt!(tag!("-")) >>
        number: map_res!(digit, i32::from_str) >>
        (sign.map(|_| -1).unwrap_or(1) * number)
    )
);

named!(parse_operator<&str, Operator>,
    alt!(
        tag!("plus") => {|_| Operator::Addition } |
        tag!("divided by") => {|_| Operator::Division } |
        tag!("multiplied by") => {|_| Operator::Multiplication } |
        tag!("minus") => {|_| Operator::Subtraction }
    )
);

named!(parse_expression<&str, Expression>,
    do_parse!(
        init: parse_number >>
        rest:
            fold_many0!(
                do_parse!(
                    space >>
                    op: parse_operator >>
                    space >>
                    rhs: parse_number >>
                    ((op, rhs))
                ),
                Expression { lhs: init, op: Operator::Addition, rhs: 0 },
                |acc: Expression, (op, rhs)| Expression { lhs: acc.eval(), op: op, rhs }
            ) >>
        (rest)
    )
);

named!(parse_command<&str, Expression>,
    do_parse!(
        tag!("What is") >>
        space >>
        expression: parse_expression >>
        tag!("?") >>
        (expression)
    )
);


pub struct  WordProblem<'a> {
    command: &'a str,
}

impl<'a> WordProblem<'a> {

    pub fn new(command: &'a str) -> Self {
        WordProblem { command }
    }

    pub fn answer(&self) -> Result<i32, ()> {
        let expression = match parse_command(self.command) {
            IResult::Done(_, expr) => expr,
            _ => return Err(()),
        };

        Ok(expression.eval())
    }

}
