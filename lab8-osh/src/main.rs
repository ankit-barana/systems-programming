mod input;
use libc;
mod pipeline;
use input::split_input;
use input::InputToken;
use input::Result;
use std::io;
use std::io::Write;


 mod signals {
    use std::io;
    use std::sync::atomic::{AtomicBool, Ordering};
    static INTERRUPTED: AtomicBool = AtomicBool::new(false);

    pub fn install_signal_handlers() -> io::Result<()> {
        unsafe {
            let action = libc::sigaction {
                sa_sigaction: handler as libc::sighandler_t,
                ..std::mem::zeroed()
            };

            if libc::sigaction(libc::SIGINT, &action, std::ptr::null_mut()) < 0 {
                return Err(io::Error::last_os_error());
            }

            if libc::sigaction(libc::SIGQUIT, &action, std::ptr::null_mut()) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    extern "C" fn handler(_sig: libc::c_int) {
        INTERRUPTED.store(true, Ordering::Relaxed);
    }

    pub fn was_interrupted() -> bool {
        let was_interrupted = INTERRUPTED.load(Ordering::Relaxed);
        INTERRUPTED.swap(false, Ordering::Relaxed);
        was_interrupted
    }

    pub fn read_line() -> std::io::Result<String> {
        use std::io::BufRead;
        let mut stdin = std::io::stdin().lock();
        let mut buf = Vec::new();
    
        loop {
            let (done, used) = {
                let available = stdin.fill_buf()?;
                match available.iter().position(|&b| b == b'\n') {
                    Some(i) => {
                        buf.extend_from_slice(&available[..=i]);
                        (true, i + 1)
                    }
                    None => {
                        buf.extend_from_slice(available);
                        (false, available.len())
                    }
                }
            };
            stdin.consume(used);
            if done || used == 0 {
                let s = String::from_utf8(buf)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
                return Ok(s);
            }
        }
    }
}


fn run_shell() -> Result<()> {

    'inner: loop {
        if signals::was_interrupted() {
            println!(); 
        }
        print!("$ ");
        std::io::stdout().flush().unwrap();

        let signal_read = signals::read_line();
        match signal_read {
            Err(err) if err.kind() == std::io::ErrorKind::Interrupted => {
                continue;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
            _ => {
                break 'inner;
            }
        }
    }

    // print!("$ ");
    // std::io::stdout().flush().unwrap();

    // input is collected into this variable
    let mut input = String::new();

    // closes the program if no input was passed (Ctrl-D was pressed)
    let input_length = io::stdin().read_line(&mut input)?;

    if input_length == 0 {
        println!("");
        std::process::exit(0);
    }

    // continue to ask for input if only spaces were provided
    if input.trim().is_empty() {
        println!("Please provide a valid Input");
        return Ok(());
    }

    // parses the input into a pipeline
    let cmd_input = split_input(&input).unwrap();
    match pipeline::Pipeline::new(&cmd_input) {
        Ok(pipeline) => {
            // run the pipeline instead
            pipeline.run().expect("Error running pipeline");
        }
        Err(err) => {
            eprintln!("Error creating a Pipeline: {}", err);
        }
    } 
    Ok(())
}

fn main() {
    signals::install_signal_handlers();
    println!("Welcome to the Oberlin Shell!");
    loop {
        if let Ok(()) = run_shell() {
            continue;
        }
    }
}


// 'inner: loop {
//     if signals::was_interrupted() {
//         println!(); 
//     }
//     print!("$ ");
//     std::io::stdout().flush().unwrap();

//     let signal_read = signals::read_line();
//     match signal_read {
//         Err(err) if err.kind() == std::io::ErrorKind::Interrupted => {
//             continue;
//         }
//         Err(err) => {
//             eprintln!("Error: {}", err);
//         }
//         _ => {
//             break 'inner;
//         }
//     }
// }
