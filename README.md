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

## Troubleshooting

`can't open Hardware dll`:

Download https://openhardwaremonitor.org/downloads/ and move contents into OpenHardwareMonitor - replacing all files and folders.

---

`link.exe not found`:


1\. Install Visual Studio Build Tools 2019

2\. Import exported configuration file linked here with the following components installed:

- Windows Universal CRT SDK
- C++ 2019 Redistributable MSMs
- MSVC v142 - VS 2019 C++ x64/x86 build tools (Spectre-mitigated as well)

---

```
thelio-io-windows-master\wix\main.wxs(63) : error LGHT0129 : Cannot open the merge module 'VCRedist' from file 'msvc\Microsoft_VC142_CRT_x64.msm'.
Error[1] (Command): The 'light' application failed with exit code = 129
Traceback (most recent call last):
  File "C:\Workspaces\thelio-io-windows-master\build.py", line 16, in <module>
    subprocess.check_call([
  File "C:\Python310\lib\subprocess.py", line 369, in check_call
    raise CalledProcessError(retcode, cmd)
subprocess.CalledProcessError: Command '['cargo', 'wix', '--nocapture', '--verbose']' returned non-zero exit status 1.
```

Copy `C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Redist\MSVC\v142\MergeModules\Microsoft_VC142_CRT_x64.msm` to `.\msvc\Microsoft_VC142_CRT_x64.msm`

---

```
unsupported sys_vendor 'System76' and product_version 'thelio-r1'
Custom {
    kind: Other,
    error: "unsupported sys_vendor \'System76\' and product_version \'thelio-r1\'",
}
```

1\. Update this section of `main.rs ~ln 89`:

```
("System76", "thelio-r1") => {
    debug!("{} {} uses standard fan curve", sys_vendor, product_version);
    FanCurve::standard()
},
```

System76 Thelio Builds:

- thelio-r1
- thelio-mira-r1

2\. Rebuild Windows Service:

```
rm -rf target && python build.py
```

Note that the build may fail here but re-running `python build.py` succeeds
