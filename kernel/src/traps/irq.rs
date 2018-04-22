use pi::interrupt::Interrupt;

use traps::TrapFrame;

pub fn handle_irq(interrupt: Interrupt, tf: &mut TrapFrame) {
    //acknowledge and setup a new timer interrupt
    match interrupt {
        Interrupt::Timer1 => {
            //read the compare register and modify for next interrupt
            use pi::timer;
            use process::TICK;
            timer::Timer::new().tick_in( TICK );
        },
        _ => {},
    }
}
