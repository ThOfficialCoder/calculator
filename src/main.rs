enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

fn main() {
    let result = calculate(Operators::Add, 10.0, 10.0);
    println!("{}", result);
}

fn calculate(operator: Operators, firstNum: f32, secondNum: f32) -> f32 {
    match operator {
        Operators::Add => firstNum + secondNum,
        Operators::Subtract => firstNum - secondNum,
        Operators::Multiply => -firstNum * secondNum,
        Operators::Divide => firstNum / secondNum,
        Operators::Modulo => firstNum % secondNum,
    }
}
