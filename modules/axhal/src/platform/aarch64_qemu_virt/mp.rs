use crate::mem::{virt_to_phys, PhysAddr};

/// Starts the given secondary CPU with its boot stack.
pub fn start_secondary_cpu(cpu_id: usize, stack_top: PhysAddr) {
    extern "C" {
        fn _start_secondary();
    }
    let entry = virt_to_phys(va!(_start_secondary as usize));
    crate::platform::aarch64_common::psci::cpu_on(cpu_id, entry.as_usize(), stack_top.as_usize());
}

/// Converts the given CPU hardware ID to its logical ID.
pub fn cpu_hard_id_to_logic_id(hard_id: usize) -> usize {
    hard_id
}
