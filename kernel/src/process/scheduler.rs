use std::collections::VecDeque;

use mutex::Mutex;
use process::{Process, State, Id};
use traps::TrapFrame;

/// The `tick` time.
// FIXME: When you're ready, change this to something more reasonable.
pub const TICK: u32 = 2 * 1000 * 1000;

/// Process scheduler for the entire machine.
#[derive(Debug)]
pub struct GlobalScheduler(Mutex<Option<Scheduler>>);

impl GlobalScheduler {
    /// Returns an uninitialized wrapper around a local scheduler.
    pub const fn uninitialized() -> GlobalScheduler {
        GlobalScheduler(Mutex::new(None))
    }

    /// Adds a process to the scheduler's queue and returns that process's ID.
    /// For more details, see the documentation on `Scheduler::add()`.
    pub fn add(&self, process: Process) -> Option<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").add(process)
    }

    /// Performs a context switch using `tf` by setting the state of the current
    /// process to `new_state`, saving `tf` into the current process, and
    /// restoring the next process's trap frame into `tf`. For more details, see
    /// the documentation on `Scheduler::switch()`.
    #[must_use]
    pub fn switch(&self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").switch(new_state, tf)
    }

    /// Initializes the scheduler and starts executing processes in user space
    /// using timer interrupt based preemptive scheduling. This method should
    /// not return under normal conditions.
    pub fn start(&self) {
        
        use func_shell;

        //first process setup
        let mut trap_frame_ptr;
        let p = match Process::new() {
            Some( mut x) => {
                //manually setup trap frame for the first process
                x.trap_frame.ELR = func_shell as u64;
                x.trap_frame.SP = x.stack.top().as_u64(); //SP to top of the process stack
                x.trap_frame.SPSR = x.trap_frame.SPSR & !( ( ( 0b1111 as u64 ) << 6 ) ); //clear interrupt mask bits DAIF
                trap_frame_ptr = x.trap_frame.clone();
                x
            },
            _ => { panic!( "first process creation" ); },
        };

        let mut s = Scheduler::new();
        *self.0.lock() = Some( s );

        //enable timer 1 interrupt
        use pi::interrupt;
        interrupt::Controller::new().enable( interrupt::Interrupt::Timer1 );
        //set timer interrupt value
        use pi::timer;
        timer::tick_in( TICK );
        
        match self.add( p ) {
            Some( id ) => {},
            _ => { panic!( "first process scheduling" ); },
        };            

        // skip continuing to HANDLER after context_restore because there isn't any other process
        // context to be restored from the stack
        // x0 and x30 should also be reset since it is not in context_restore
        unsafe {
            asm!("mov SP, $0
                  bl context_restore
                  ldr x0, =_start
                  mov SP, x0
                  mov x0, #0
                  mov x30, #0
                  eret" //jump back to EL0 at func_shell
                 :: "r"(trap_frame_ptr)
                 :: "volatile");
        }
    }
}

#[derive(Debug)]
struct Scheduler {
    processes: VecDeque<Process>,
    current: Option<Id>,
    last_id: Option<Id>,
}

impl Scheduler {
    /// Returns a new `Scheduler` with an empty queue.
    fn new() -> Scheduler {
        Scheduler {
            processes: VecDeque::new(),
            current: None,
            last_id: None,
        }
    }

    /// Adds a process to the scheduler's queue and returns that process's ID if
    /// a new process can be scheduled. The process ID is newly allocated for
    /// the process and saved in its `trap_frame`. If no further processes can
    /// be scheduled, returns `None`.
    ///
    /// If this is the first process added, it is marked as the current process.
    /// It is the caller's responsibility to ensure that the first time `switch`
    /// is called, that process is executing on the CPU.
    fn add(&mut self, mut process: Process) -> Option<Id> {
        let id_generate = match self.last_id {
            None => 0,
            Some(x) => x + 1,
        };

        if self.current.is_none() && id_generate == 0 {
            self.current = Some( 0 );
        }

        process.trap_frame.TPIDR = id_generate;

        self.processes.push_back( process );

        Some( id_generate )
    }

    /// Sets the current process's state to `new_state`, finds the next process
    /// to switch to, and performs the context switch on `tf` by saving `tf`
    /// into the current process and restoring the next process's trap frame
    /// into `tf`. If there is no current process, returns `None`. Otherwise,
    /// returns `Some` of the process ID that was context switched into `tf`.
    ///
    /// This method blocks until there is a process to switch to, conserving
    /// energy as much as possible in the interim.
    fn switch(&mut self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        unimplemented!("Scheduler::switch()")
    }
}
