#![allow(dead_code)]      // suppresses unused warning
use std::io;

fn main() {
    // let data = vec![2, 3, 5, 7, 11, 13]; // vec! creates rsizable arrays. Similar to arrayList in Python
    // let mut data2 = Vec::new();  // when making a new one don't use macro and capitaliza the first letter
    // data2.push(2);
    // data2.push(3);
    // data2.push(5);
    // for x in &data {  // typically, first for loop consumes our Vec and hence Vec no longer exists - This only happens in Rust
    //     println!("{x}");    // causes the loop to interate over a refernce to the Vec, rather then vec itself 
    // }
    // for x in &data {  // use &vec to prevent use-up of the existing Vector
    //     println!("{x}");
    // }
    println!("Input: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Output:\n{}", clown_case(&input.trim()));  // Without trim() the last clwon emoji is added in a new line (which is added to input when we click enter) 

}

// generates a list with elements arraged in reversed order
fn reversed_vec(input_data: &[i32]) -> Vec<i32> {  // whatever input we get is converted into a reference

    let mut result: Vec<i32> = Vec::new();
    for i in input_data.iter().rev() {
        result.push(*i);          // * converts &i32 into i32
    }
    return result;
}

// checks if the numbers in a list are in order
fn is_in_order(data: &[i32]) -> bool {
    let mut prev: i32 = i32::MIN;
    for x in data {
        if *x >= prev {  
            prev =  *x;  // & converts $i32 to i32
            continue
        } else {
            return false;
        }
    }
    return true;
}
// returns the sum of all the numbers in a Vector
fn manual_sum(data: &[i32]) -> i32 {
    let mut data_iter = data.iter();
    let mut sum = 0;
    loop {
        let num: Option<&i32> = data_iter.next();
        if num.is_none() {
            return sum;
        } if num.is_some() {
            sum = sum + num.unwrap();
        }
    }
}

// returns a String with alternate Capitalization and includes a clown emoji at the start and end.
// if the string is empty, returns a single clown
fn clown_case(s: &str) -> String {
    let mut num = 0;
    let mut result = String::new();

    if s.is_empty() {
        result.extend("🤡".chars());
    } else {
        result.extend("🤡 ".chars());
        for ch in s.chars() {
            if ch.is_alphabetic() && num == 0 {
                result.extend(ch.to_lowercase());
                num = 1;
            } else if ch.is_alphabetic() && num == 1 {
                result.extend(ch.to_uppercase());
                num = 0;
            } else {
                result.extend(std::iter::once(ch));
            }
        } 
        result.extend(" 🤡".chars()); 
    } return result; 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reversed_vec_empty() {
        let data = vec![];
        assert_eq!(reversed_vec(&data), vec![]);
    }

    #[test]
    fn reversed_vec_one() {
        let data = vec![2];
        assert_eq!(reversed_vec(&data), vec![2]);
    }

    #[test]
    fn reversed_vec_three() {
        let data = vec![1, 2, 3];
        assert_eq!(reversed_vec(&data), vec![3, 2, 1]);
    }

    #[test]
    fn is_in_order_empty() {
        let data = vec![];
        assert!(is_in_order(&data));
    }
    
    #[test]
    fn is_in_order_one() {
        let data = vec![1];
        assert!(is_in_order(&data));
    }
    
    #[test]
    fn is_in_order_multiple_in_order() {
        let data = vec![1, 2, 3];
        assert!(is_in_order(&data));
    }

    #[test]
    fn is_in_order_multiple_out_of_order() {
        let data = vec![10, -3, 8, 2, 25];
        assert!(!is_in_order(&data));    // use ! to assert negations
    }

    #[test]
    fn manual_sum_empty() {
        let data = vec![];
        assert_eq!(manual_sum(&data), 0);
    }

    #[test]
    fn manual_sum_one() {
        let data = vec![2];
        assert_eq!(manual_sum(&data), 2);
    }

    #[test]
    fn manual_sum_multiple() {
        let data = vec![3, 4, 5, 1, 2];
        assert_eq!(manual_sum(&data), 15);
    }

    #[test]
    fn clown_case_empty() {
        let s = "";
        assert_eq!(clown_case(s), "🤡");
    }

    #[test]
    fn clown_case_normal() {
        let s = "I'm just asking questions";
        assert_eq!(clown_case(s), "🤡 i'M jUsT aSkInG qUeStIoNs 🤡");
    }

    #[test]
    fn clown_case_greek() {
        let s = "Μην είσαι κλόουν στα ελληνικά!";
        assert_eq!(clown_case(s), "🤡 μΗν ΕίΣαΙ κΛόΟυΝ σΤα ΕλΛηΝιΚά! 🤡");
    }
}
