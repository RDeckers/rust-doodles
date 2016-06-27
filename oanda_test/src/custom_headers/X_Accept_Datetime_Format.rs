extern crate hyper;
use self::hyper::header::{ETag,EntityTag, Header, HeaderFormat, parsing};
use self::hyper::{Result,Error};
use std::fmt;
use std::str;

#[derive(Debug, Clone, Copy)]
pub enum Dateformat{
  Unix,
  RFC3339
}

#[derive(Debug, Clone, Copy)]
pub struct AcceptDateTime(Dateformat);
impl Header for AcceptDateTime{
  fn header_name() -> &'static str{
    "X-Accept-Datetime-Format"
  }

  fn parse_header(raw: &[Vec<u8>]) -> Result<AcceptDateTime>  { //Option?
    if raw.len() == 1{
      let decoded = str::from_utf8(&raw[0]);
      if decoded.is_ok(){
        match decoded.unwrap(){
          "RFC3339" => return Ok(AcceptDateTime(Dateformat::RFC3339)),
          "UNIX" => return Ok(AcceptDateTime(Dateformat::Unix)),
          _ => return Err(Error::Header)
        }
      }
    }
  Err(Error::Header)
  }
}

impl HeaderFormat for AcceptDateTime{
  fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.0 {
      Dateformat::RFC3339 => f.write_str("RFC3339"),
      Dateformat::Unix => f.write_str("UNIX")
      //_ => Err(fmt::Error)
    }
  }
}
