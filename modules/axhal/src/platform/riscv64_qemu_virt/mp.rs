use crate::mem::{virt_to_phys, PhysAddr};

/// Starts the given secondary CPU with its boot stack.
pub fn start_secondary_cpu(hartid: usize, stack_top: PhysAddr) {
    extern "C" {
        fn _start_secondary();
    }
    if sbi_rt::probe_extension(sbi_rt::Hsm).is_unavailable() {
        warn!("HSM SBI extension is not supported for current SEE.");
        return;
    }
    let entry = virt_to_phys(va!(_start_secondary as usize));
    sbi_rt::hart_start(hartid, entry.as_usize(), stack_top.as_usize());
}

/// Converts the given CPU hardware ID to its logical ID.
pub fn cpu_hard_id_to_logic_id(hard_id: usize) -> usize {
    hard_id
}
