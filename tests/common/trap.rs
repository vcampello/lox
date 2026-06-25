use lox::Lox;
use std::{cell::RefCell, fmt::Display, io::Write, rc::Rc};

/// Test mock for stdout. Pass it to the lox interpreter to trap its output.
/// # Example
/// ```
/// use lox::*;
/// mod common;
///
/// let (trap, mut rt) = Trap::new_runtime();
/// rt.run(r#"print "hello world!";"#).unwrap();
/// assert_eq!(trap.to_string(), "hello world!\n");
/// ```
#[derive(Default, Clone)]
pub struct Trap(Rc<RefCell<Vec<u8>>>);

impl Trap {
    pub fn new_runtime() -> (Trap, Lox) {
        let trap = Trap::default();
        (trap.clone(), Lox::with_writer(Box::new(trap)))
    }
}

impl Display for Trap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = self.0.borrow().to_vec();
        let s = String::from_utf8(content).expect("failed to capture stdout from interpreter");
        write!(f, "{s}")
    }
}

impl Write for Trap {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
