use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git branch")
        .output()
        .expect("Failed to run command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.to_string();

    println!("{}", s);

}
