use std::{fs::{self, File}, os::unix::prelude::MetadataExt};

use super::Result;

#[derive (Debug)]
pub struct Process {
    pub pid: i32,
    pub command_name: String,
    pub state: char,
    pub ppid: i32,
    pub session: i32,
    pub tty: i32,
    pub time: i64,
    pub cmd: Vec<String>,
    pub uid: i32,
}

impl Process {

    // takes a PID as an argument and returns a Result<Process>.
    pub fn for_pid(pid: i32) -> Result<Self> {

        let mut fields: Vec<&str> = Vec::new();

        let path = format!("/proc/{pid}/stat");
        let stat = &fs::read_to_string(&path).unwrap();
        // ok_or_else converts some into Ok if find succeeds and none into Err(message)
        let comm_start = stat.find('(').ok_or_else(|| format!("Couldn't parse stat file for PID"))?;
        let comm_end = stat.find(')').ok_or_else(|| format!("Couldn't parse stat file for PID {pid}"))?;
        
        let pid_as_str = &stat[..comm_start].trim();
        let command_name = &stat[comm_start + 1..comm_end];
        let mut remaining_fields = stat[comm_end + 1..].trim().split_whitespace().collect();

        fields.push(pid_as_str);
        fields.push(command_name);   
        fields.append(&mut remaining_fields);


        //calculates the time
        let utime: i64 = fields[14].parse()?;
        let stime: i64 = fields[15].parse()?;
        let execution_time = utime + stime;

        // looks for User ID
        let user_path = format!("/proc/{pid}/stat");
        let user_info = File::open(&user_path).map_err(|err| format!("{user_path}: {err}"))?;
        let metadata = user_info.metadata().map_err(|err| format!("{user_path}: {err}"))?;
        let uid = metadata.uid() as i32;

        // gets the process state
        let char_vec: Vec<char> = fields[2].chars().collect();
        let state = char_vec[0];

        //finds the cmdline
        let cmdline_path = format!("/proc/{pid}/cmdline");
        let cmdline_info = fs::read_to_string(cmdline_path)?;
        let cmd: Vec<String> = cmdline_info.split_terminator('\0').map(String::from).collect();


        let proc = Process {
            pid: fields[0].parse()?,
            command_name: fields[1].to_string(),
            state: state,
            ppid: fields[3].parse()?,
            time: execution_time,
            uid: uid,
            session: fields[5].parse()?,
            tty: fields[6].parse()?,
            cmd: cmd,
        };
        return Ok(proc);
    }

    /// Look up information for the current process.
    pub fn for_self() -> Result<Self> {
        let pid = std::process::id() as i32;
        return Process::for_pid(pid);
    }

    /// Returns `true` if the process is a session leader.
    pub fn is_session_leader(&self) -> bool {
        return self.pid == self.session;
    }

    /// Returns `true` if the process has a controlling terminal.
    pub fn has_tty(&self) -> bool {
        return self.tty != 0;
    }

    /// Returns a list of all running processes.
    pub fn all_processes() -> Result<Vec<Self>> {
        // intializes the output
        let mut proc_vec: Vec<Process> = Vec::new();

        for entry in fs::read_dir("/proc")? {
            // Unwrap the result and return any errors that result from reading /proc.
            let entry = entry.unwrap();

            // if the entry is not a directory or reading its metadata fails, we continue to the next iteration
            match entry.metadata() {
                Ok(metadata) if metadata.is_dir() => (),
                _ => continue,
            }

            let file_name = entry.file_name();
            
            if let Some(file_str) = file_name.to_str() {
                if let Ok(pid) = file_str.parse::<i32>() {
                    if let Ok(proc) = Process::for_pid(pid) {
                        proc_vec.push(proc);
                    }
                }
            }
        }

        return Ok(proc_vec);
        
    }
}