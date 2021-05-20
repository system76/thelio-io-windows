# Thelio Io Windows Driver (WORK IN PROGRESS)

- Install GIT LFS prior to cloning this repository
- Install Rust from https://rustup.rs/
- Install Chocolaty from https://chocolatey.org/install
- Launch an Administrator Command Prompt and run the following:
```
choco install dotnet netfx-4.8 python3 wixtoolset
```
- Launch a normal Command Prompt and run the following:
```
cargo install cargo-wix
```
- Run the following to build the installer:
```
python build.py
```
- Execute the installer at `target/wix/thelio-io-0.1.0-x86_64.msi`
- The installer will start the `System76 Thelio Io` service
- Logs can be viewed in `Event Viewer` under `Windows Logs/Application` with the
  source `System76 Thelio Io`
