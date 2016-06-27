extern crate hyper;
use self::hyper::header::{ETag,EntityTag};
use self::hyper::*;
use super::custom_headers::X_Accept_Datetime_Format::*;
use std::thread;
use std::io::{Read, copy, stdout};

pub struct PriceWatcher{
  base_url : String,
  client: Client,
  timeformat: Dateformat, //TODO: naming, make optional to reduce packet size.
  tag : ETag
}

impl PriceWatcher{
  /*fn create_request(&'a mut self){
    self.request = Some(self.client.get(&*self.base_url).header(AcceptDateTime(self.timeformat)));
  }*/
  pub fn new(currency_pairs : &[&'static  str]) -> Self{
      let mut url = String::from("http://api-sandbox.oanda.com/v1/prices?instruments=") + &String::from(currency_pairs[0]);
      let spacer = String::from("%2C");
      for i in 1..currency_pairs.len(){
          url = url + &spacer + &String::from(currency_pairs[i]);
      }
    PriceWatcher{
      base_url : url,
      client: Client::new(),
      timeformat: Dateformat::Unix,
      tag: ETag(EntityTag::new(false, String::from("")))
    }
  }
  pub fn set_timeformat(&mut self, format : Dateformat){ self.timeformat = format;}

  pub fn wait_for_update(&mut self, sleep_timer_ms : u32){
    //let request = self.create_request(); //TODO: buffer request in some way?
    //let mut request = self.client.get(&self.base_url);
    let mut res = self.client.get(&self.base_url).send().unwrap();//TODO: add Etag to request
    while *res.headers.get::<ETag>().unwrap() == self.tag{
      println!("Got the same tag: {}", self.tag);
      thread::sleep_ms(sleep_timer_ms);
      res = self.client.get(&self.base_url).send().unwrap();//TODO: add Etag to request
    }
    self.tag = (*res.headers.get::<ETag>().unwrap()).clone();
    copy(&mut res, &mut stdout()).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{}\n", body);
    //println!("{}\n{}\n{}", res.status, res.headers, body);
    //self.history.append()
  }
}
