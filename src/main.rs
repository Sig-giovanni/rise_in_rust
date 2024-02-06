fn main() {
    let  string_a = String::from("Giga");
    let string_b = String::from("Chad");

    let concatenated_string = concatenate_strings( string_a, &string_b);

    println!("{}", concatenated_string);

    let calculation = Operation::Add{ a: 5.0, b: 6.0 };
    let calculation_result = calculate_operation(calculation);
}

//Exercise One
fn concatenate_strings(s1:  String, s2:  &String)  -> String {
   let mut result = s1.clone();
    result.push_str(s2);
    result
}


//Exercise Two
enum Operation{
    Add{a: f64, b: f64},
    Subtract{a: f64, b: f64},
    Multiply{a: f64, b: f64},
    Divide{a: f64, b: f64},
}

fn calculate_operation(operand: Operation) {
    match operand {
        Operation::Add { a, b } => {
            println!("{} + {} = {}", a, b, a + b);
        }
        Operation::Subtract { a, b } => {
            println!("{} - {} = {}", a, b, a - b);
    }
    Operation::Multiply { a, b } => {
        println!("{} * {} = {}", a, b, a * b)
    }
    Operation::Divide { a, b } => {
        println!("{} / {} = {}", a, b, a / b);
    }
}
}