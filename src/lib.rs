use core::time::Duration;

use std::io;

use psutil::cpu::CpuPercentCollector;

pub const DURATION_DEFAULT: Duration = Duration::from_millis(1_000);

pub fn get_cpus_percent<S>(duration: Duration, sleep: S) -> Result<Vec<f32>, io::Error>
where
    S: Fn(Duration),
{
    let mut col: CpuPercentCollector = CpuPercentCollector::new().map_err(io::Error::other)?;
    sleep(duration);
    col.cpu_percent_percpu().map_err(io::Error::other)
}

pub fn get_cpus_percent_default() -> Result<Vec<f32>, io::Error> {
    get_cpus_percent(DURATION_DEFAULT, std::thread::sleep)
}

pub fn cpus_percent_print(values: &[f32]) -> Result<(), io::Error> {
    for pair in values.iter().enumerate() {
        let (ix, val) = pair;
        println!("cpu{ix}:{val}");
    }
    Ok(())
}

pub fn cpus_percent_print_avg(values: &[f32]) -> Result<(), io::Error> {
    let sum: f32 = values.iter().sum();
    let cnt: usize = values.len();
    let avg: f32 = sum / (cnt as f32);
    println!("{avg}");
    Ok(())
}
