#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct TrapFrame {
    pub ELR: u64,
    pub SPSR: u64,
    pub SP: u64,
    pub TPIDR: u64,
    pub q: [ u128; 32 ],
    pub x1_x29: [ u64; 29 ],
    pub _reserved: u64,
    pub x30: u64,
    pub x0: u64,
}
