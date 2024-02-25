use::process::Result;
use std::fs;                                                                           


// "0.00 0.01 0.04 5/756 26030"
//  1-3 => load averages
//  4 (numerator) => runnable kernel scheduling entity
//  4 (denominator) => number of kernel scheduling entities
// 5 = PID

/**
 * WRITING BETTER ERROR MESSAGES
 * command.map_error(|err| format!("{}: {err}", path.display())) - writes the error message with the respective path
 */


/// Returns the number of currently runnable kernel scheduling entities
/// (processes and threads) and  the total number of kernel scheduling entities.
fn scheduling_entities() -> Result<(usize, usize)> {

    let f = &fs::read_to_string("/proc/loadavg")?; // read the pseudo file
    let parts: Vec<&str> = f.split_whitespace().collect(); // collect the pseudo file int a vector
    let ratio: Vec<&str> = parts[3].split("/").collect(); // split the the 4th value at '/' 
    let runnable = ratio[0].parse(); // the value before / is runnable count
    let total = ratio[1].parse(); // the one after is kernal scheduling entities count
    return Ok((runnable.unwrap(), total.unwrap()));
}




fn main() {
    match scheduling_entities() {
        Ok((runnable, total)) => {
            println!("Runnable entities {runnable} of {total}");
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
