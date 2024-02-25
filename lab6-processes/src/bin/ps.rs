use clap::Parser;
use process::Process;

#[derive(Debug, Parser)]
#[command(author, version, about = "ps - report process status", long_about = None)]
struct Config {
    /// Write information for all processes.
    #[arg(short = 'A', short_alias = 'e')]
    all: bool,
    
    /// the process has a controlling terminal and the process is not a session leader, 
    /// then print the process.
    #[arg(short = 'a')]
    opt_a: bool,

    /// Write information for all processes, except session leaders
    #[arg(short = 'd')]
    opt_d: bool,

    /// Generate a full listing.
    #[arg(short = 'f')]
    opt_f: bool,

    /// Generate a long listing.
    #[arg(short = 'l')]
    opt_l: bool,
}

// filters the process based on the option
fn filter_processes (procs: Vec<Process>, config: &Config) -> Vec<Process>{
    let mut filtered_procs = Vec::new();

    for proc in procs {
        if config.all {
            return filtered_procs;
        } else if config.opt_a && proc.has_tty() && !proc.is_session_leader() {
            filtered_procs.push(proc);
        } else if config.opt_d && !proc.is_session_leader() {
            filtered_procs.push(proc);
        } else {
            if proc.uid == Process::for_self().unwrap().uid && proc.tty == Process::for_self().unwrap().tty {
                filtered_procs.push(proc);
            }
        }
    }
    return filtered_procs;
}


// prints the output
fn print_table (procs: Vec<Process>, config: &Config) {

    if config.opt_l {
        println!("{:1} {:<15} {:<8} {:<8} {:<8} {:<8} {}", "S", "UID", "PID", "PPID", "TTY", "TIME", "CMD");
    } else if config.opt_f {
        println!("{:<15} {:<8} {:<8} {:<8} {:<8} {}", "UID", "PID", "PPID", "TTY", "TIME", "CMD");
    } else {
        println!("{:<8} {:<8} {:<8} {}", "PID", "TTY", "TIME", "CMD");
    }

    for proc in procs {

        let cmd: String = if !config.opt_f {
            proc.command_name
        } else if proc.cmd.is_empty() && config.opt_f {
            format!("[{}]", proc.command_name)
        } else {
            proc.cmd.join(" ")
        };

        // gets all the values
        let (s, uid, pdi, ppid, tty, time) = (proc.state, proc.uid, proc.pid, proc.ppid, proc.tty, proc.time);

        if config.opt_l {
            println!("{:1} {:<15} {:<8} {:<8} {:<8} {:<8} {}", s, uid, pdi, ppid, tty, time, cmd);
        } else if config.opt_f {
            println!("{:<15} {:<8} {:<8} {:<8} {:<8} {}", uid, pdi, ppid, tty, time, cmd);
        } else {
            println!("{:<8} {:<8} {:<8} {}", pdi, tty, time, cmd);
        }
    }
}

fn main() {
    let config = Config::parse();
    let filtered_procs = filter_processes(Process::all_processes().unwrap(), &config);
    print_table(filtered_procs, &config);
}
