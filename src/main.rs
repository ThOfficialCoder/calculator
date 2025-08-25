use std::io;

enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

fn main() {
    let mut first_input = String::new();
    let mut second_input = String::new();

    println!("Enter first number:");

    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read line");

    println!("Enter operator (+, -, *, /, %): ");
    let mut operator_input = String::new();

    io::stdin()
        .read_line(&mut operator_input)
        .expect("Failed to read line");

    println!("Enter second number: ");

    io::stdin()
        .read_line(&mut second_input)
        .expect("Failed to read line");

    let num1 = first_input
        .trim()
        .parse()
        .expect("Not a integer");

    let num2 = second_input
        .trim()
        .parse()
        .expect("Not a integer");

    let add = calculate(Operators::Add, num1, num2);
    let subtract = calculate(Operators::Subtract, num1, num2);
    let multiply = calculate(Operators::Multiply, num1, num2);
    let divide = calculate(Operators::Divide, num1, num2);
    let modulo = calculate(Operators::Modulo, num1, num2);

    if num2 == 0.0 {
        println!("Cannot divide by zero");
    } 

    match operator_input.trim() {
        "+" => println!("{} + {} is {}", num1, num2, add),
        "-" => println!("{} - {} is {}", num1, num2, subtract),
        "*" => println!("{} * {} is {}", num1, num2, multiply),
        "/" => println!("{} / {} is {}", num1, num2, divide),
        "%" => println!("{} % {} is {}", num1, num2, modulo),
        _ => println!("Unknown operator"),
    }

    
}

fn calculate(operator: Operators, first_num: f32, second_num: f32) -> f32 {

    match operator {
        Operators::Add => first_num + second_num,
        Operators::Subtract => first_num - second_num,
        Operators::Multiply => first_num * second_num,
        Operators::Divide => first_num / second_num,
        Operators::Modulo => first_num % second_num,
    }
}
