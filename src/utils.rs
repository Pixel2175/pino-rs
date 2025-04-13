use std::process::{Command, Stdio};

pub fn is_running(name: &str) -> bool {
    let output = Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .stdout(Stdio::piped())  
        .stderr(Stdio::null())   
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let pids = String::from_utf8_lossy(&output.stdout);
            pids.lines().count() == 2
        },
        _ => false 
    }
}
