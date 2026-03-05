# Installing flare
1. Install rust from [Rust](https://rust-lang.org)
2. On windows(if using PowerShell) run [installation script](install.ps1) or at Mac and linux run

# Building flare
If you really wanna do it yourself, and i mean on windows you need to do it yourself. Here are the steps:

1. Install rust from [Rust](https://rust-lang.org)

2. Clone repo 
```bash
$ git clone https://github.com/DomioKing653/Flare
$ cd Flare
```
3. build project using rust's cargo
```bash
$ cargo build --bin flarec --release
$ cargo build --bin flauncher --release

```
4. Put ```./target/release/flarec.exe``` to enviroment variables

5. than you can run ```flarec build ...``` from anywhere
