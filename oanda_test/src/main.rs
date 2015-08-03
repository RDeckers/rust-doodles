#[macro_use] extern crate hyper;
header! { (XAcceptDatetimeFormat, "X-Accept-Datetime-Format") => [String] }

use hyper::*;
use std::fmt;
use hyper::client::{RequestBuilder, IntoUrl};
use hyper::header::{ETag,EntityTag, Header, HeaderFormat};
use std::io::Read;

#[derive(Debug, Clone, Copy)]
enum Dateformat{
  Unix,
  RFC3339
}

#[derive(Debug, Clone, Copy)]
struct AcceptDateTime(Dateformat);

impl Header for AcceptDateTime{
  fn header_name() -> &'static str{
    "X-Accept-Datetime-Format"
  }

  fn parse_header(raw: &[Vec<u8>]) -> Option<AcceptDateTime> { //Option?
    if raw.len() == 1{
      let decoded = std::str::from_utf8(&raw[0]);
      if decoded.is_ok(){
        match decoded.unwrap(){
          "RFC3339" => return Some(AcceptDateTime(Dateformat::RFC3339)),
          "UNIX" => return Some(AcceptDateTime(Dateformat::Unix)),
          _ => return None
        }
      }
    }
  //Err(hyper::Error::Header)
  None
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

struct PriceWatcher{
  base_url : String,
  history : Vec<(String, f64)>,
  client: Client,
  timeformat: Dateformat, //TODO: naming, make optional to reduce packet size.
  tag : ETag
}

impl PriceWatcher{
  /*fn create_request(&'a mut self){
    self.request = Some(self.client.get(&*self.base_url).header(AcceptDateTime(self.timeformat)));
  }*/
  pub fn new(currency_pair : &'static  str) -> Self{
    PriceWatcher{
      base_url : String::from("http://api-sandbox.oanda.com/v1/prices?instruments=") + &String::from(currency_pair),
      history : Vec::new(),
      client: Client::new(),
      timeformat: Dateformat::Unix,
      tag: ETag(EntityTag::new(false, String::from("")))
    }
  }
  pub fn set_timeformat(&mut self, format : Dateformat){ self.timeformat = format;}
  pub fn wait_for_update(&mut self, sleep_timer_ms : u32){
    //let request = self.create_request(); //TODO: buffer request in some way?
    let mut res = self.request.unwrap().send().unwrap();//TODO: add Etag to request
    while *res.headers.get::<ETag>().unwrap() == self.tag{
      println!("Got the same tag: {}", self.tag);
      std::thread::sleep_ms(sleep_timer_ms);
      res = self.request.unwrap().send().unwrap();//TODO: add Etag to request
    }
    self.tag = (*res.headers.get::<ETag>().unwrap()).clone();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{}\n{}\n{}", res.status, res.headers, body);
    //self.history.append()
  }
}

fn main() {
  let mut eur_sek = PriceWatcher::new("EUR_SEK");
  for _ in 0..10{
    eur_sek.wait_for_update(10_000);
    std::thread::sleep_ms(10_000);
  }
  let mut client = Client::new();
  let mut res = client.get("http://api-sandbox.oanda.com/v1/prices?instruments=EUR_SEK").send().unwrap();
  // Read the Response.
  let mut body = String::new();
  res.read_to_string(&mut body).unwrap();
  println!("{}\n{}\n{}", res.status, res.headers, body);
}

/*
build own request for client. store it. 
static, pooled, http-connector? http://hyper.rs/hyper/hyper/client/pool/struct.Pool.html
*/
