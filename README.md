# GVPaint
Paint ported to Windows, macOS, GNU/Linux, BSD and others from [PekOS-GV](https://github.com/StjepanBM1/PekOS/tree/3.X-Kernel/PekOS/PekOS%20GV)

[Guide](https://github.com/Andrej123456789/GVPaint/blob/master/GUIDE.md) on how to use text files to draw painting

[GVPaint 1.0.0](https://github.com/Andrej123456789/GVPaint/releases/tag/v1.0.0)

## Features (or bugs)
- Written fully in Rust (so expect little slower compile time when compiling first time)
- Memory safety
- 10 colors (black, dark blue, light green, light cyan, light magenta, brown, light grey, yellow and white)
- Saves paintings to .txt and .png files
    - it saves and opens only files with name `painting`
    - opening .png files are only for read only purpose
    - when opening .png files make sure your canvas size is larger than on saved image
- TUI app (runs fully in terminal/terminal emulator)
- Under MIT license

## Compile & Run
- Install [Rust](https://www.rust-lang.org/tools/install)
- Run `cargo run --release`
- If you use install (GNU/)Linux based operating system make sure you have `cc` and `gcc` compiler installed

**Thanks to [StjepanBM1](https://github.com/StjepanBM1) for creating one of best paint programs in the world :)**
