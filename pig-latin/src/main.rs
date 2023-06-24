use std::io;

fn main() {
    println!("Hello and welcome to pig latin converter!");
    let sentence = get_input();

    let mut result = String::new();

    let word_arr = sentence.split_whitespace();
    let mut is_first = true;

    for word in word_arr {
        if is_first {
            result = format!("{}",convert_to_pig_latin(word));
            is_first = false;
            continue;
        }
        result = format!("{} {}", result, convert_to_pig_latin(word));
    }
    println!("{}", result);
}

fn convert_to_pig_latin(word: &str) -> String {
    let vowels = "aeiou";
    let word = word.to_lowercase();
    
    if vowels.contains(&word[0..1]) {
        return format!("{}-hay", word);
    } else {
        return format!("{}-{}{}", &word[1..word.len()], &word[0..1], "ay");
    }
}

fn get_input() -> String {
    println!("Please enter a sentence to convert: ");

    let user_input = loop {
        let mut result = String::new();

        io::stdin()
            .read_line(&mut result)
            .expect("Something went horribly wrong");

        let result = match result
            .trim()
            .parse() {
                Ok(string) => string,
                Err(_) => {
                    println!("Please enter a valid sentence");
                    continue;
                }
            };
        break result;
    };
    user_input
}

