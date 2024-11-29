use axconfig::CPU_ID_LIST;

use crate::mem::{virt_to_phys, PhysAddr};

extern "C" {
    fn _start_secondary();
}

pub fn cpu_hard_id_to_logic_id(hard_id: usize) -> usize {
    if CPU_ID_LIST.is_empty() {
        hard_id
    } else {
        CPU_ID_LIST.iter().position(|&x| x == hard_id).unwrap()
    }
}

/// Starts the given secondary CPU with its boot stack.
pub fn start_secondary_cpu(cpu_id: usize, stack_top: PhysAddr) {
    extern "C" {
        fn _start_secondary();
    }
    let entry = virt_to_phys(va!(_start_secondary as usize));
    crate::platform::aarch64_common::psci::cpu_on(
        CPU_ID_LIST[cpu_id],
        entry.as_usize(),
        stack_top.as_usize(),
    );
}
