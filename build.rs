use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed=app.manifest");
    let mut res = winres::WindowsResource::new();
    res.set_manifest(include_str!("app.manifest"));
    res.compile().expect("failed to add app manifest");

    println!("cargo:rerun-if-changed=wrapper/Program.cs");
    println!("cargo:rerun-if-changed=wrapper/wrapper.csproj");
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
