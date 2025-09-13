
pub fn string_to_f64(input: String) -> f64 {
    return input.trim().parse().expect("Please enter a valid number");
}

pub fn get_input() -> String {

    use std::io;
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    return input;
}

pub fn get_input_with_prompt(prompt: &str) -> String {
    println!("{}", prompt);
    get_input()
}