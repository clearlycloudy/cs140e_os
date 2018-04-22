use traps::TrapFrame;

/// Sleep for `ms` milliseconds.
///
/// This system call takes one parameter: the number of milliseconds to sleep.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the approximate true elapsed time from when `sleep` was called to
/// when `sleep` returned.
pub fn sleep(ms: u32, tf: &mut TrapFrame) {

    use console::kprintln;
    use pi::timer;
    use process;
    use SCHEDULER;
    use std::ptr;
    
    let t_us = ( ms as u64 ) * 1000;
    let t_start = timer::current_time();
    let t_expect_end = t_start + t_us;
    
    //do not reschedule until poll succeeds
    SCHEDULER.switch( process::State::Waiting(
        Box::new( move |p| {
            let t_now = timer::current_time();
            let diff : u64 = t_now.saturating_sub( t_expect_end );
            if  diff >= t_us {
                p.trap_frame.x0 = (diff / 1000); //return approx sleep time in ms
                p.trap_frame.x1_x29[6] = 0; //set return status to success
                true
            } else {
                false
            }
        })
    ), tf );
}

///to be called by userland
pub fn syscall_sleep_ms( ms: u32 ) -> u32 {

    let mut ret : u64;
    let mut err_code : u64;

    //input: sleep time
    //output: actual sleep time, error code
    unsafe {
        asm!("mov x0, $2
              svc 1
              mov $0, x0
              mov $1, x7"
             : "=r"(ret), "=r"(err_code) : "r"(ms) :"x0","x7": );
    }

    if err_code != 0 {
        panic!("syscall sleep failed");
    }

    ret as u32
}

pub fn handle_syscall(num: u16, tf: &mut TrapFrame) {
    match num {
        1 => { //sleep
            let t_ms = tf.x0;
            sleep( t_ms as u32, tf );
        },
        _ => {},
    }
}

