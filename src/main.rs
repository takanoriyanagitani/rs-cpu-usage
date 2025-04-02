use core::time::Duration;

use std::process::ExitCode;

use std::io;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(io::Error::other)
}

fn duration() -> Duration {
    env_val_by_key("ENV_DURATION_MS")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .map(Duration::from_millis)
        .unwrap_or(rs_cpu_usage::DURATION_DEFAULT)
}

fn cpus_percent() -> Result<Vec<f32>, io::Error> {
    rs_cpu_usage::get_cpus_percent(duration(), std::thread::sleep)
}

fn print_avg() -> bool {
    env_val_by_key("ENV_PRINT_AVERAGE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(true)
}

fn cpus_percent2stdout() -> Result<(), io::Error> {
    let values: Vec<f32> = cpus_percent()?;
    rs_cpu_usage::cpus_percent_print(&values)
}

fn cpus_percent2stdout_avg() -> Result<(), io::Error> {
    let values: Vec<f32> = cpus_percent()?;
    rs_cpu_usage::cpus_percent_print_avg(&values)
}

fn cpuinfo2stdout() -> Result<(), io::Error> {
    match print_avg() {
        true => cpus_percent2stdout_avg(),
        false => cpus_percent2stdout(),
    }
}

fn main() -> ExitCode {
    cpuinfo2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
