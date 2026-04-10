use vertex::runtime::virtual_machine::virtual_machine::VM;
#[no_mangle]
pub extern "C" fn vm_entry(ptr: *const u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    } 
    //SAFETY:It's safe because we now that the compiler saved the bytes in correct order
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    
    match VM::from_bytes(bytes.to_vec()) {
        Ok(mut vm) => {
            if let Err(e) = vm.run() {
                eprintln!("VM runtime error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("VM loading error: {}", e);
        }
    }
}
