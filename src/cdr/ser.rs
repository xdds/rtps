use serde::ser::{Serialize, Serializer, SeqVisitor, MapVisitor};
//use serde_json;

use serde::ser::Error as SerErr;
use std::error::Error as Err;

use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

use std::fmt::{ Display, Formatter };
use std::fmt::Error as FmtError;
//use std::io::Error as IOError;

use std::io::Write;

#[derive(Debug)]
pub enum CdrEndianness {
    Little,
    Big
}

#[derive(Debug)]
pub struct CdrError{
    pub reason: String
}

impl Display for CdrError {
    fn fmt(&self, _: &mut Formatter) -> Result<(), FmtError> {
        unimplemented!();
    }
}

impl Err for CdrError {
    fn description(&self) -> &str {
        "NO"
    }

    fn cause(&self) -> Option<&Err> {
        None
    }
}

impl SerErr for CdrError {
    fn custom<T: Into<String>>(_: T) -> Self {
        CdrError{
            reason: "fdsa".to_string()
        }
    }
}

pub struct CdrSerializer<W> where W: Write {
    pub endianness: CdrEndianness,
    pub write_handle: W
}

impl<W: Write> Serializer for CdrSerializer<W> {
    type Error = CdrError;

    fn serialize_bool(&mut self, _ /* v */: bool) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_i64(&mut self, _ /* v */: i64) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        match self.endianness {
            CdrEndianness::Little => {
                match self.write_handle.write_u32::<LittleEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(CdrError{
                        reason: "dddd".to_string()
                    })
                }
            },
            CdrEndianness::Big => {
                match self.write_handle.write_u32::<BigEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(CdrError{
                        reason: "dddd".to_string()
                    })
                }
            }
        }

    }
    fn serialize_u64(&mut self, _ /* v */: u64) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_f64(&mut self, _ /* v */: f64) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> {
        match self.write_handle.write_all(value.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(CdrError{
                reason: err.description().to_string()
            })
        }
    }
    fn serialize_unit(&mut self) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_none(&mut self) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_some<V> (&mut self, _ /* value */: V) -> Result<(), Self::Error> /* where V: Serializer */ {
        unimplemented!();
    }
    fn serialize_seq<V>(&mut self, mut visitor: V) -> Result<(), Self::Error> where V: SeqVisitor {
        try!(match visitor.len() {
            Some(len) => self.serialize_u32(len as u32),
            None => Err(CdrError{
                reason: "could not write seq len".to_string()
            })
        });

        loop {
            match visitor.visit( self ) {
                Ok(Some(_)) => {
                    continue
                },
                Ok(None) => {
                    break
                },
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
    fn serialize_seq_elt<T>(&mut self, value: T) -> Result<(), Self::Error> where T: Serialize {
//        let reason_str = serde_json::to_string(&value).unwrap();
//        debug!("{}", reason_str);
//
//        //        Err(CdrError{
//        //            reason: reason_str
//        //        })
        value.serialize(self)
    }
    fn serialize_map<V>(&mut self, _ /* visitor */: V) -> Result<(), Self::Error> where V: MapVisitor {
        unimplemented!();
    }
    fn serialize_map_elt<K, V>(&mut self, /* key */_: K, /* value */ _: V) -> Result<(), Self::Error> where K: Serialize, V: Serialize {
        unimplemented!();
    }

    fn serialize_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        let buf = [v];

        match self.write_handle.write(&buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(CdrError{
                reason: err.description().to_string()
            })
        }
    }

    fn serialize_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        match self.endianness {
            CdrEndianness::Little => {
                match self.write_handle.write_u16::<LittleEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CdrError{
                        reason: err.description().to_string()
                    })
                }
            },
            CdrEndianness::Big => {
                match self.write_handle.write_u16::<BigEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CdrError{
                        reason: err.description().to_string()
                    })
                }
            }
        }
    }
}
