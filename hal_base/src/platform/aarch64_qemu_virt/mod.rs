pub const PSCI_0_2_FN_BASE: u32 = 0x84000000;
pub const PSCI_0_2_FN_SYSTEM_OFF: u32 = PSCI_0_2_FN_BASE + 8;

pub fn terminate() -> ! {
    unsafe {
        core::arch::asm!(
            "hvc #0",
            in("x0") PSCI_0_2_FN_SYSTEM_OFF,
        )
    }
    loop {}
}
