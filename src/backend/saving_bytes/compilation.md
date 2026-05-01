# !This is documentation for developers only!
In this document, you will find how we are compiling .vtx files to executables with [zig toolchain](https://ziglang.org/).
The main logic is at [./compile_tools.rs]. There we are lexing, parsing, building and compiling the file.
## Main logic
We are compiling our vm using (wrapper)[../../../codegen/]. It will compile ```libvm_runtime.a``` and we'll be using it to link it to the final
executable file. This should be prebuilded and be somewhere in path becouse ```vertexC``` will try to find it. After this we are going
to compile the code to bytcodes and save them to ```out/name```. Then we create ```tmp_launcher.zig```:
```zig
const std = @import("std");
extern fn vm_entry(ptr: [*]const u8, len: usize) void;
var program = @embedfile("path/to/bytcode");
pub fn main() !void {
  vm_entry(program.ptr, program.len);
}
```
Once it's saved we are linking it with:
```bash
zig build-exe tmp_launcher_path.zig runtime_path -lc -lunwind -Doptimize=ReleaseSmall -femit-bin=out/bin/out_file_name
```
After its compiled we remove the temporary launcher   `tmp_launcher.zig` with:
```rust    
fs::remove_file(tmp_launcher_path).unwrap();
```
And the compilation process is finished
