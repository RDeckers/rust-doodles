extern crate serde_json;
use serde_json::Value;
use serde_json::de::from_reader;
struct InstrumentTick{
    time : u64,
    bid : f64, //TODO: Decimal types?
    ask : f64
}

impl InstrumentTick{
    fn from_json_object() -> InstrumentTick{

    }
}

pub struct InstrumentTicker{
    name : String,
    ticks : Vec<InstrumentTick>
}
