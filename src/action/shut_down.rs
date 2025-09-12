use std::process::Command;

pub fn shut_down() {
    let status = Command::new("shutdown")
        .arg("-h")
        .arg("now")
        .status()
        .expect("failed to execute shutdown command");

    println!("Shutdown command exited with: {:?}", status);
}
