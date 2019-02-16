mod retry;

use std::time::Duration;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

use retry::retry;


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
                .takes_value(true)
                .default_value("5"),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .value_name("INTERVAL")
                .help("Interval in seconds between the retries")
                .takes_value(true)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("exit_code")
                .short("c")
                .long("exitcode")
                .value_name("EXIT_CODE")
                .help("The expected exit code to stop retrying")
                .takes_value(true)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Be quiet. Suppress command output"),
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
    let exitcode = match matches.value_of("exit_code").unwrap_or_default().parse() {
        Ok(c) => c,
        Err(_) => panic!("The given exit code option must be an Integer"),
    };
    let quiet = matches.is_present("quiet");
    let cmd: Vec<&str> = matches.values_of("command").unwrap().collect();

    let config = retry::RetryConfig {
        max: max_retries,
        interval: interval,
        expected_exitcode: exitcode,
        quiet: quiet,
        cmd: cmd,
    };

    retry(config);
}
