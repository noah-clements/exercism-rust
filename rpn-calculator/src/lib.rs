// refactored after seeing good community solution from @ledo01

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => {
                stack.push(*val);
            },
            _ => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                match input {
                    CalculatorInput::Add => stack.push(a + b),
                    CalculatorInput::Subtract => stack.push(b - a),
                    CalculatorInput::Multiply => stack.push(a * b),
                    CalculatorInput::Divide => stack.push(b / a),
                    _ => return None,
                }
            }
        }
    }
    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
