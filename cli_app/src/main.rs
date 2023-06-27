use std::io;

mod parsemath;
use parsemath::ast;
use parsemath::parser::{ParseError, Parser};

fn main() {
    println!("Hello, world!,Welcome to the Arithetic Expression Evaluator");
    println!("Enter your arithmetic expression below");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => {
                        println!("Error in evaluating the expression");
                    }
                };
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    //remove whitespace chars
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}
