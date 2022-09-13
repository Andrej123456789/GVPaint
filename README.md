# GVPaint
Paint ported to Windows (only in WSL though), macOS, GNU/Linux, BSD and others from [PekOS-GV](https://github.com/StjepanBM1/PekOS/tree/3.X-Kernel/PekOS/PekOS%20GV)

[Guide](https://github.com/Andrej123456789/GVPaint/blob/master/GUIDE.md) on how to use text files to draw painting

[GVPaint 1.0.0](https://github.com/Andrej123456789/GVPaint/releases/tag/v1.0.0)

## Features (or bugs)
- Written fully in Rust (so expect little slower compile time when compiling first time)
- Memory safety
- Speed and resource usage (RAM usage is about 760 Kilobytes, can be larger as painting is larger)
- Has 9 colors (including black, grey, red, green, blue, aqua, yellow, orange, white)
- TUI app (runs fully in terminal/terminal emulator)
- Under MIT license

## Compile & Run
- Install [Rust](https://www.rust-lang.org/tools/install)
- Type in terminal/terminal emulator: `cargo run --release`
- If you use install (GNU/)Linux based operating system make sure you have `cc` and `gcc` compiler installed

**Thanks to [StjepanBM1](https://github.com/StjepanBM1) for creating one of best paint programs in the world :)**
