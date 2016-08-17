use std::rc::Rc;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct RcBuffer {
    buf: Rc<Vec<u8>>
}

impl RcBuffer {
    pub fn from_vec(vec: Vec<u8>) -> Self {
        RcBuffer { buf: Rc::new(vec) }
    }

    pub fn buf(&self) -> &[u8] {
        let borrow : &Vec<u8> = self.buf.borrow();
        borrow.borrow()
    }
}