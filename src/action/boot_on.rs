use chrono::{DateTime, Local};
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn read_uptime_secs() -> io::Result<f64> {
    let text = fs::read_to_string("/proc/uptime")?;
    let first = text.split_whitespace().next().unwrap_or("0");
    first
        .parse::<f64>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn compute_boot_time() -> io::Result<String> {
    let uptime_secs = read_uptime_secs()?;
    let now = SystemTime::now();
    let dur = Duration::from_secs_f64(uptime_secs);
    let boot_time = now.checked_sub(dur).unwrap_or(UNIX_EPOCH);

    let datetime: DateTime<Local> = boot_time.into();
    Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub fn check_boot_id() -> (bool, String) {
    let boot_id = fs::read_to_string("/proc/sys/kernel/random/boot_id").unwrap();
    let boot_time = compute_boot_time().unwrap();

    let file_path = "./last_boot_id.txt";
    if Path::new(file_path).exists() {
        let prev_boot_id = fs::read_to_string(file_path).unwrap();
        if prev_boot_id.trim() == boot_id.trim() {
            println!("Program restarted (same boot).");
            return (false, boot_time);
        } else {
            println!("New boot detected!, original is {prev_boot_id}, new is {boot_id}");
        }
    } else {
        println!("First run since boot.");
    }

    fs::write(file_path, boot_id).unwrap();
    (true, boot_time)
}
