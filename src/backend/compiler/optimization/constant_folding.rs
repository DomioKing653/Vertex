use crate::backend::compiler::instructions::Instructions;
use std::collections::HashMap;

/// Performs constant folding optimization and returns both the optimized code
/// and a mapping from old instruction indices to new indices.
///
/// This function combines adjacent constant operations (like PushNumber + PushNumber + Add)
/// into single instructions while tracking how instruction positions change.
///
/// # Returns
/// A tuple of (optimized_code, old_to_new_mapping)
pub fn constant_folding(code: Vec<Instructions>) -> (Vec<Instructions>, HashMap<usize, usize>) {
    let mut out = Vec::new();
    let mut old_to_new: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    while i < code.len() {
        match (code.get(i), code.get(i + 1), code.get(i + 2)) {
            (
                Some(Instructions::PushNumber(a)),
                Some(Instructions::PushNumber(b)),
                Some(Instructions::Add),
            ) => {
                // Map all three old indices to the single new index
                old_to_new.insert(i, out.len());
                old_to_new.insert(i + 1, out.len());
                old_to_new.insert(i + 2, out.len());

                out.push(Instructions::PushNumber(a + b));
                i += 3;
            }
            (
                Some(Instructions::PushNumber(a)),
                Some(Instructions::PushNumber(b)),
                Some(Instructions::Sub),
            ) => {
                // Map all three old indices to the single new index
                old_to_new.insert(i, out.len());
                old_to_new.insert(i + 1, out.len());
                old_to_new.insert(i + 2, out.len());

                out.push(Instructions::PushNumber(a - b));
                i += 3;
            }
            (
                Some(Instructions::PushNumber(a)),
                Some(Instructions::PushNumber(b)),
                Some(Instructions::Mul),
            ) => {
                // Map all three old indices to the single new index
                old_to_new.insert(i, out.len());
                old_to_new.insert(i + 1, out.len());
                old_to_new.insert(i + 2, out.len());

                out.push(Instructions::PushNumber(a * b));
                i += 3;
            }
            (
                Some(Instructions::PushNumber(a)),
                Some(Instructions::PushNumber(b)),
                Some(Instructions::Div),
            ) => {
                // Map all three old indices to the single new index
                old_to_new.insert(i, out.len());
                old_to_new.insert(i + 1, out.len());
                old_to_new.insert(i + 2, out.len());

                out.push(Instructions::PushNumber(a * b));
                i += 3;
            }
            (
                Some(Instructions::PushNumber(a)),
                Some(Instructions::PushNumber(b)),
                Some(Instructions::Modulo),
            ) => {
                // Map all three old indices to the single new index
                old_to_new.insert(i, out.len());
                old_to_new.insert(i + 1, out.len());
                old_to_new.insert(i + 2, out.len());

                out.push(Instructions::PushNumber(a % b));
                i += 3;
            }

            _ => {
                old_to_new.insert(i, out.len());
                out.push(code[i].clone());
                i += 1;
            }
        }
    }

    (out, old_to_new)
}
