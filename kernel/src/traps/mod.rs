mod irq;
mod trap_frame;
mod syndrome;
mod syscall;

use pi::interrupt::{Controller, Interrupt};

pub use self::trap_frame::TrapFrame;

use console::kprintln;
use self::syndrome::Syndrome;
use self::irq::handle_irq;
use self::syscall::handle_syscall;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Source {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}

/// This function is called when an exception occurs. The `info` parameter
/// specifies the source and kind of exception that has occurred. The `esr` is
/// the value of the exception syndrome register. Finally, `tf` is a pointer to
/// the trap frame for the exception.
#[no_mangle]
pub extern fn handle_exception(info: Info, esr: u32, tf: &mut TrapFrame) {
    use aarch64;
    use shell;
    use fs::FileSystem;
    use FILE_SYSTEM;

    match info.kind {
        Kind::Synchronous => {

            // for the cpase of synchronous instruction other than system calls such as brk,
            // the CPU stores the address of instruction that generates the exception
            // in ELR_ELx.
            // Thus, to set address to the next instruction (32-bit wide) upon exception return, it is ELR_ELx + 4
            tf.ELR += 4;

            //ESR_ELx is valid if it's a synchronous exception
            let syndrome = Syndrome::from( esr );
            match syndrome {
                Syndrome::Brk(x) => {
                    
                    kprintln!( "exception: brk: {:?}", x );
                    
                    shell::shell( "!brk>", & FILE_SYSTEM );
                },
                Syndrome::Svc(x) => {
                    
                    kprintln!( "exception: Svc: {:?}", x );
                    
                    shell::shell( "!svc>", & FILE_SYSTEM );
                },
                _ => {},
            }       
        },
        Kind::Irq => {

            //forward interrupts such as ones coming from timers
            
            //kprintln!( "exception: irq: {:?}", info );

            if Controller::new().is_pending( Interrupt::Timer1 ) {
                handle_irq( Interrupt::Timer1, tf );
            }
        }
        _ => {},
    }
}
