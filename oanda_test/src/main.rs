pub mod pricewatcher;
pub mod custom_headers;
use pricewatcher::*;
use custom_headers::X_Accept_Datetime_Format::*;

use std::fmt;
use std::io::Read;


fn main() {
  let mut eur_sek = PriceWatcher::new(&["EUR_SEK","EUR_USD"]);
  eur_sek.set_timeformat(Dateformat::Unix);
  for _ in 0..10{
    eur_sek.wait_for_update(10_000);
  }
  /*let mut client = Client::new();
  let mut res = client.get("http://api-sandbox.oanda.com/v1/prices?instruments=EUR_SEK").send().unwrap();
  // Read the Response.
  let mut body = String::new();
  res.read_to_string(&mut body).unwrap();
  println!("{}\n{}\n{}", res.status, res.headers, body);
  */
}

/*
build own request for client. store it.
static, pooled, http-connector? http://hyper.rs/hyper/hyper/client/pool/struct.Pool.html
*/
