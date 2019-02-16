use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct RetryConfig<'a> {
    pub max: u32,
    pub interval: Duration,
    pub expected_exitcode: i32,
    pub quiet: bool,
    pub cmd: Vec<&'a str>,
}

pub fn retry(config: RetryConfig) {
    let mut i = 1;
    while i <= config.max || config.max == 0 {
        let status = match Command::new(&config.cmd[0])
            .args(&config.cmd[1..config.cmd.len()])
            .stdin(if config.quiet {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .stdout(if config.quiet {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .stderr(if config.quiet {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .status()
        {
            Ok(s) => s,
            Err(err) => panic!("Failed to execute command: {}", err),
        };

        match status.code() {
            Some(code) if code == config.expected_exitcode => {
                println!("Successfully ran command. Abort retry.");
                break;
            }
            Some(code) => println!("[Retry {}] Command failed with exit code {}", i, code),
            None => println!(
                "[Retry {}] Command failed because it was termianted by a signal",
                i
            ),
        }

        if i != config.max {
            thread::sleep(config.interval);
        }

        i += 1;
    }
}
