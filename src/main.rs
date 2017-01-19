extern crate rustc_serialize;
extern crate hyper;
use rustc_serialize::json;
use std::{env,clone};
use std::iter::Iterator;

#[derive(RustcDecodable,Debug)]
#[allow(non_snake_case)]
pub struct Arrival {
    DESTINATION: String,
    DIRECTION: char,
    EVENT_TIME: String,
    LINE: String,
    STATION: String,
    TRAIN_ID: String,
    WAITING_SECONDS: String,
    WAITING_TIME: String,
}

fn main() {
    let apikey = env::var("MARTA_API_KEY").unwrap();
    let arrivals = get_arrivals(&apikey).unwrap();
    // println!("{:?}", arrivals);
    assert!(String::from("DORAVILLE STATION") == "DORAVILLE STATION");
    
    let doraville_s = arrivals.into_iter().filter(|arr: &Arrival| -> bool {
        arr.STATION == "DORAVILLE STATION"
    });
    for arr in doraville_s {
        println!("{:?}", arr);
    }
    
    // let midtown_n = arrivals.into_iter().filter(|arr: &Arrival| -> bool {
    //     arr.STATION == "MIDTOWN STATION" && arr.DIRECTION == 'N'
    // });
    // for arr in midtown_n {
    //     println!("{:?}", arr);
    // }
                                                
    // match get_arrivals(&apikey) {
    //     Some(arrivals) => println!("{:?}", arrivals),
    //     None => return,
    // }
}

#[allow(unused_must_use)]
fn get_arrivals(apikey: &str) -> Option<Vec<Arrival>> {
    use hyper::{client, Url};
    use std::io::Read;

    let mut rta = Url::parse("http://developer.itsmarta.\
                              com/RealtimeTrain/RestServiceNextTrain/GetRealtimeArrivals")
        .unwrap();
    rta.query_pairs_mut().append_pair("apikey", &apikey);

    let mut buf = String::new();

    let cli = client::Client::new();
    let mut res = match cli.get(rta).send() {
        Ok(x) => x, 
       Err(_) => return None,
    };
    res.read_to_string(&mut buf);

    match json::decode(&buf) {
        Ok(arrivals) => arrivals,
        Err(_) => None,
    }
}
