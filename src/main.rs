fn main() {
    let  string_a = String::from("Giga");
    let string_b = String::from("Chad");

    let concatenated_string = concatenate_strings( string_a, &string_b);

    println!("{}", concatenated_string);
}

fn concatenate_strings(s1:  String, s2:  &String)  -> String {
   let mut result = s1.clone();
    result.push_str(s2);
    result
}