/// Struct that defines a callback that can be passed to a process. The callback
/// takes four arguments that are `Driver` and callback specific, so they are
/// represented generically here.
///
/// Likely these four arguments will get passed as the first four register
/// values, but this is architecture-dependent.
#[derive(Copy, Clone)]
pub struct FunctionCall {
    pub argument0: usize,
    pub argument1: usize,
    pub argument2: usize,
    pub argument3: usize,
    pub pc: usize,
}

use core::fmt;

impl fmt::Debug for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
            "FunctionCall arg0:{:x}\targ1:{:x}\targ2:{:x}\targ3:{:x}\tpc:{:x}\t", 
            self.argument0, self.argument1, self.argument2, self.argument3, self.pc)
    }
}

