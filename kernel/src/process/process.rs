use traps::TrapFrame;
use process::{State, Stack};

/// Type alias for the type of a process ID.
pub type Id = u64;

/// A structure that represents the complete state of a process.
#[derive(Debug)]
pub struct Process {
    /// The saved trap frame of a process.
    pub trap_frame: Box<TrapFrame>,
    /// The memory allocation used for the process's stack.
    pub stack: Stack,
    /// The scheduling state of the process.
    pub state: State,
}

impl Process {
    /// Creates a new process with a zeroed `TrapFrame` (the default), a zeroed
    /// stack of the default size, and a state of `Ready`.
    ///
    /// If enough memory could not be allocated to start the process, returns
    /// `None`. Otherwise returns `Some` of the new `Process`.
    pub fn new() -> Option<Process> {
        let s = match Stack::new() {
            Some(x) => x,
            _ => { return None },
        };
        Some( Process {
            trap_frame: Box::new(TrapFrame::default()),
            stack: s,
            state: State::Ready,
        } )
    }

    /// Returns `true` if this process is ready to be scheduled.
    ///
    /// This functions returns `true` only if one of the following holds:
    ///
    ///   * The state is currently `Ready`.
    ///
    ///   * An event being waited for has arrived.
    ///
    ///     If the process is currently waiting, the corresponding event
    ///     function is polled to determine if the event being waiting for has
    ///     occured. If it has, the state is switched to `Ready` and this
    ///     function returns `true`.
    ///
    /// Returns `false` in all other cases.
    pub fn is_ready(&mut self) -> bool {
        
        use std::mem;

        let mut s = mem::replace( & mut self.state, State::Ready );

        let is_ready = match s {
            State::Ready => {
                true
            },
            State::Waiting(ref mut poll_fn) => {
                poll_fn( self )
            },
            _ => {
                false
            },
        };

        if is_ready {
            self.state = State::Ready;
        } else {
            //swap back
            mem::replace( & mut self.state, s );
        }
        
        is_ready
    }
}
