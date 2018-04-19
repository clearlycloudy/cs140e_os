#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Fault {
    AddressSize,
    Translation,
    AccessFlag,
    Permission,
    Alignment,
    TlbConflict,
    Other(u8)
}

impl From<u32> for Fault {
    fn from(val: u32) -> Fault {
        use self::Fault::*;
        let instr_fault_status_code = val & ( ( 1 << 6 ) - 1 );
        match instr_fault_status_code {
            0b000000...0b000011 => AddressSize,
            0b000100...0b000111 => Translation,
            0b001000...0b001011 => AccessFlag,
            0b001101...0b001111 => Permission,
            0b100001 => Alignment,
            0b110000 => TlbConflict,
            x => Other(x as u8),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Syndrome {
    Unknown,
    WfiWfe,
    McrMrc,
    McrrMrrc,
    LdcStc,
    SimdFp,
    Vmrs,
    Mrrc,
    IllegalExecutionState,
    Svc(u16),
    Hvc(u16),
    Smc(u16),
    MsrMrsSystem,
    InstructionAbort {
        kind: Fault,
        level: u8,
    },
    PCAlignmentFault,
    DataAbort {
        kind: Fault,
        level: u8
    },
    SpAlignmentFault,
    TrappedFpu,
    SError,
    Breakpoint,
    Step,
    Watchpoint,
    Brk(u16),
    Other(u32)
}

/// Converts a raw syndrome value (ESR) into a `Syndrome` (ref: D1.10.4).
impl From<u32> for Syndrome {
    fn from(esr: u32) -> Syndrome {
        use self::Syndrome::*;

        //get exception class bits[31:26]
        let exception_class : u8 = ( ( esr >> 26 ) & 0xFF ) as u8;
        
        //instruction length bit[25]
        let instruction_length = ( esr >> 25 ) & 0b1;

        //instruction specific syndrome bits[24:0]
        let instruction_specific : u32 = esr & ( (1 << 25) - 1 );

        let syndrome : Syndrome = match exception_class {
            0b000000 => Unknown,
            0b000001 => WfiWfe,
            0b000011 => McrMrc,
            0b000100 => McrMrc,
            0b000110 => LdcStc,
            0b000111 => SimdFp,
            0b001000 => Vmrs,
            0b001001 => Other(0b001001),
            0b001100 => Mrrc,
            0b001110 => IllegalExecutionState,
            0b010001 | 0b010101 => Svc( instruction_specific as u16),
            0b010010 | 0b010110 => Hvc( instruction_specific as u16),
            0b010011 | 0b010111 => Smc( instruction_specific as u16),
            0b011000 => MsrMrsSystem,
            0b011001 => Other(instruction_specific),
            0b011010 => Other(instruction_specific),
            0b011111 => Other(instruction_specific),
            0b100000 => InstructionAbort { //from lower exception level
                kind: Fault::from(instruction_specific),
                level: 0 },
            0b100001 => InstructionAbort { //same level
                kind: Fault::from(instruction_specific),
                level: 1 },
            0b100010 => PCAlignmentFault,
            0b100100 => DataAbort { //from lower level
                kind: Fault::from(instruction_specific),
                level: 0 },
            0b100101 => DataAbort { //same level
                kind: Fault::from(instruction_specific),
                level: 1 },
            0b100110 => SpAlignmentFault,
            0b101000 | 0b101100=> TrappedFpu, //arch32/64
            0b101111 => SError,
            0b110000 | 0b110001 => Breakpoint, //lower/same exception level
            0b110010 | 0b110011 => Step, //lower/same exception level
            0b110100 | 0b110101 => Watchpoint, //lower/same exception level
            0b111000 => Brk( ( instruction_specific & ((1 << 16)-1) ) as u16), //arch32
            0b111010 => Other(exception_class as u32),
            0b111100 => Brk( ( instruction_specific & ((1 << 16)-1) ) as u16), //arch64
            x => Other(x as u32),
        };

        syndrome
    }
}
