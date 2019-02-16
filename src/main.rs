use std::process::Command;
use std::thread;
use std::time::Duration;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

#[derive(Debug)]
struct RetryConfig<'a> {
    max: u32,
    interval: Duration,
    cmd: Vec<&'a str>,
}

fn retry(config: RetryConfig) {
    let mut i = 1;
    while i <= config.max || config.max == 0 {
        let status = Command::new(&config.cmd[0])
            .args(&config.cmd[1..config.cmd.len()])
            .status()
            .unwrap();

        if status.success() {
            println!("Successfully ran command. Abort retry.");
            break;
        } else {
            match status.code() {
                Some(code) => println!("[Retry {}] Command failed with exit code {}", i, code),
                None => println!(
                    "[Retry {}] Command failed because it was termianted by a signal",
                    i
                ),
            }
        }

        if i != config.max {
            thread::sleep(config.interval);
        }

        i += 1;
    }
}

fn main() {
    let matches = App::new("retry")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("max")
                .short("m")
                .long("max")
                .value_name("MAX_RETRIES")
                .help("Maximum retries. Use 0 for unlimited retries")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .value_name("INTERVAL")
                .help("Interval in seconds between the retries")
                .takes_value(true),
        )
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("command")
                .value_name("COMMAND")
                .help("Command to run")
                .required(true)
                .multiple(true)
                .last(true),
        )
        .get_matches();

    let max_retries: u32 = match matches.value_of("max").unwrap_or("5").parse() {
        Ok(u) => u,
        Err(_) => panic!("The given MAX option must be an Integer"),
    };
    let interval = Duration::from_secs(match matches.value_of("interval").unwrap_or("1").parse() {
        Ok(u) => u,
        Err(_) => panic!("The given INTERVAL option must be an Integer"),
    });
    let cmd: Vec<&str> = matches.values_of("command").unwrap().collect();

    let config = RetryConfig {
        max: max_retries,
        interval: interval,
        cmd: cmd,
    };

    retry(config);
}
