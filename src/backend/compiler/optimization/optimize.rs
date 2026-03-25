use crate::backend::compiler::{
    instructions::Instructions, optimization::constant_folding::constant_folding,
};
use std::collections::HashMap;

/// Optimizes a vector of instructions while preserving jump address correctness.
///
/// This function applies various optimization passes to the instruction stream,
/// such as constant folding. After each optimization, it ensures that all jump
/// addresses (Jump, JumpIfFalse) are updated to point to the correct instructions
/// in the optimized code.
///
/// # Example
/// ```
/// // Original code:
/// // 0: PushNumber(5.0)
/// // 1: PushNumber(3.0)
/// // 2: Add
/// // 3: JumpIfFalse(7)
/// // 4: PushNumber(1.0)
/// // 5: Jump(7)
/// // 6: PushNumber(0.0)
/// // 7: Halt
///
/// // After optimization (constant folding):
/// // 0: PushNumber(8.0)      // 5.0 + 3.0 folded
/// // 1: JumpIfFalse(5)        // address updated from 7 to 5
/// // 2: PushNumber(1.0)
/// // 3: Jump(5)               // address updated from 7 to 5
/// // 4: PushNumber(0.0)
/// // 5: Halt
/// ```
pub fn optimize(code: Vec<Instructions>) -> Vec<Instructions> {
    // Apply constant folding and get the index mapping
    let (code, old_to_new) = constant_folding(code);
    // Fix jump addresses based on the index mapping
    fix_jump_addresses(code.clone(), old_to_new)
}

/// Fixes all jump addresses in the code using the provided index mapping.
///
/// This function updates Jump and JumpIfFalse instructions to point to the correct
/// positions after optimization has removed or combined instructions.
///
/// # Arguments
/// * `code` - The optimized instruction vector
/// * `old_to_new` - Mapping from old instruction indices to new indices
///
/// # Returns
/// The code with all jump addresses corrected
fn fix_jump_addresses(
    code: Vec<Instructions>,
    mut old_to_new: HashMap<usize, usize>,
) -> Vec<Instructions> {
    // Handle addresses that point beyond the last instruction
    // (e.g., end of code jumps)
    let last_new_idx = code.len();

    // Find the maximum old index to handle end-of-code jumps
    if let Some(&max_old_idx) = old_to_new.keys().max() {
        old_to_new.insert(max_old_idx + 1, last_new_idx);
    }

    // Update all jump instructions with corrected addresses
    code.into_iter()
        .map(|inst| match inst {
            Instructions::Jump(old_addr) => {
                let new_addr = old_to_new.get(&old_addr).copied().unwrap_or_else(|| {
                    // If exact mapping doesn't exist, find the closest valid address
                    find_closest_new_address(&old_to_new, old_addr)
                });
                Instructions::Jump(new_addr)
            }
            Instructions::JumpIfFalse(old_addr) => {
                let new_addr = old_to_new.get(&old_addr).copied().unwrap_or_else(|| {
                    // If exact mapping doesn't exist, find the closest valid address
                    find_closest_new_address(&old_to_new, old_addr)
                });
                Instructions::JumpIfFalse(new_addr)
            }
            other => other,
        })
        .collect()
}

/// Find the closest new address for a given old address.
///
/// This handles cases where a jump points to an instruction that was optimized away.
/// When an instruction is removed during optimization, any jumps targeting it need to
/// be redirected to the next valid instruction.
///
/// # Arguments
/// * `old_to_new` - Mapping from old instruction indices to new indices
/// * `old_addr` - The original address that needs to be mapped
///
/// # Returns
/// The new address to jump to, or the original address if no mapping is found
fn find_closest_new_address(old_to_new: &HashMap<usize, usize>, old_addr: usize) -> usize {
    // Try to find the next valid instruction after the old address
    let mut search_addr = old_addr;
    loop {
        if let Some(&new_addr) = old_to_new.get(&search_addr) {
            return new_addr;
        }
        search_addr += 1;
        // Safety check: don't search forever
        if search_addr > old_addr + 100 {
            return old_addr; // Fallback to original address
        }
    }
}
