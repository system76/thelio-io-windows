use std::process::Command;

fn main() {
    let status = Command::new("dotnet")
        .arg("build")
        .arg("--configuration")
        .arg("Release")
        .current_dir("wrapper")
        .status()
        .expect("failed to build wrapper");
    if ! status.success() {
        panic!("failed to build wrapper: {}", status);
    }
}
