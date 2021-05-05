# Thelio Io Windows Driver (WORK IN PROGRESS)

- Install GIT LFS prior to cloning this repository
- Install Rust from https://rustup.rs/
- Install Chocolaty from https://chocolatey.org/install
- Launch an Administrator Command Prompt and run the following:
```
choco install dotnet netfx-4.8 wixtoolset
```
- Launch a normal Command Prompt and run the following:
```
cargo install cargo-wix
```
- Run the following to build the installer:
```
cargo wix -v --nocapture
```
- Execute the installer at `target/wix/thelio-io-0.1.0-x86_64.msi`
- Execute the program from the `bin/thelio-io.exe` file in the install directory
