use serde;

pub struct CdrSeqVisitor<'a, T> where T: Sized + 'a + serde::Deserializer  {
    pub deserializer: &'a mut T,
    pub len: usize,
    pub debug: bool
}

impl<'a,T> CdrSeqVisitor<'a,T> where T: serde::Deserializer {
    pub fn new(t: &'a mut T, len: usize, debug: bool) -> Self {
        CdrSeqVisitor{ deserializer: t, len, debug }
    }
}

impl<'a,R> serde::de::SeqVisitor for CdrSeqVisitor<'a,R> where R: serde::Deserializer {
    type Error = R::Error;

    fn visit<T>(&mut self) -> Result<Option<T>,Self::Error> where T: serde::Deserialize {
        if self.len == 0 {
            return Ok(None);
        }

        self.len -= 1;
        serde::Deserialize::deserialize(self.deserializer).map(Some)
    }

    fn end(&mut self) -> Result<(),Self::Error> {
        Ok(())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.len == 0 {
            (self.len,None)
        } else {
            (self.len,Some(self.len))
        }
    }
}