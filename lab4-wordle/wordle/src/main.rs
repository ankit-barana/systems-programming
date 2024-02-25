use std::io;
use std::io::Write;
use colored::Colorize;
mod words;  // gives access to the public functions in words.rs
           // namely is_word_valid() abd random_word()

           // use trim() to deal with newline characters

// colors 'ch' black on a green background
fn green(ch: char) -> String {
    ch.to_string()
      .black()
      .on_green()
      .to_string()
}

// colors 'ch' black on a yellow background
fn white(ch: char) -> String {
    ch.to_string()
      .black()
      .on_white()
      .to_string()
}

// colors 'ch' black on a white background
fn yellow(ch: char) -> String {
    ch.to_string()
      .black()
      .on_yellow()
      .to_string()
}


fn color_guess(guess: &str, secret: &str) -> String {

    let mut secret_iter = secret.chars();
    let mut unmatch_vec: Vec<char> = secret.chars().collect();
    let mut index = 0;
    for chr in guess.chars() {  // removes the matched chaarcters from the vector
        let secret_chr = secret_iter.next().unwrap();
        if chr == secret_chr {
            unmatch_vec.remove(index);
        } else {
            index += 1; 
        }
    }

    let mut secret_iter2 = secret.chars();
    let mut result: String = String::new();
    for ch in guess.chars() {
        let secret_ch = secret_iter2.next().unwrap();
        if ch == secret_ch {
            result.push_str(&green(ch));
        } else {
            if unmatch_vec.contains(&ch) {
                result.push_str(&yellow(ch));
                // finds the first occurance of the character and removes it and ignores if not found
                if let Some(pos) = unmatch_vec.iter().position(|x| *x == ch) {
                    unmatch_vec.remove(pos);
                    
                }
            } else {
                result.push_str(&white(ch));
            }
        }
    }
    return result;
}

fn main() {
    let secret_word = words::random_word(); // gets a random word from the list.
    let mut count = 0; 
    let mut output: String = String::new();
    loop {
        if count < 6 {
            print!("Enter your guess: ");
            std::io::stdout().flush().unwrap(); // prints the instruction 

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");

            let guess_trimmed = guess.trim().to_ascii_uppercase();
            println!("");

            if words::is_word_valid(&guess_trimmed) {
                if secret_word == guess_trimmed {
                    output.push_str(&format!("{}", color_guess(&guess_trimmed, &secret_word)));
                    println!("\n{}", &output);
                    println!("");
                    println!("Yay 🥳 You win!");
                    break;
                } else {
                    // check if the word is valid
                    // return all attempts and color accordingly
                    output.push_str(&format!("{} \n", color_guess(&guess_trimmed, &secret_word)));
                    println!("{}", &output);
                    count += 1;
                    continue;
                }
            } else {
                continue;
            }
        
        } else {
            println!("Time out 😭! The secret word is {}.", secret_word);
            break;
        } 
    }
}
