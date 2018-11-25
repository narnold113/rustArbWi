extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde_json::{Value};

#[derive(Debug)]
struct OrderBook {
    asks: Vec<[f32; 2]>,
    bids: Vec<[f32; 2]>,
    balance: f32
}

impl OrderBook {
    fn get_asks_wp(&self) -> f32 {
        let mut i = 0;
        let mut volume = 0.0;
        let mut w_price = 0.0;

        loop {
            volume += self.asks[i][0] * self.asks[i][1];
            w_price += self.asks[i][0] * ((self.asks[i][0] * self.asks[i][1]) / self.balance);

            if volume > self.balance {
                let remainder: f32 = volume - self.balance;
                w_price -= self.asks[i][0] * (remainder / self.balance);
                return w_price;
            }

            i = i + 1;
        }
    }

    fn get_bids_wp(&self) -> f32 {
        let mut i = 0;
        let mut volume = 0.0;
        let mut w_price = 0.0;

        loop {
            volume += self.bids[i][0] * self.bids[i][1];
            w_price += self.bids[i][0] * ((self.bids[i][0] * self.bids[i][1]) / self.balance);

            if volume > self.balance {
                let remainder: f32 = volume - self.balance;
                w_price -= self.bids[i][0] * (remainder / self.balance);
                return w_price;
            }

            i = i + 1;
        }
    }
}

fn main() {
    let btc_usdt_ob = create_ob_struct(String::from("https://api.binance.com/api/v1/depth?symbol=BTCUSDT&limit=50"));
    let btc_usdt_ask: f32 = btc_usdt_ob.get_asks_wp();
    let btc_usdt_bid: f32 = btc_usdt_ob.get_bids_wp();

    println!("{:?}", btc_usdt_bid);
}

fn get_order(url: String) -> String {
    reqwest::get(&url).expect("Couldn't make request.").text().expect("Couldn't read response text")
}

fn create_json_object(url: String) -> Result<Value, ()> {
    let book = get_order(url);
    let book_parsed: Value = serde_json::from_str(&book).unwrap();
    Ok(book_parsed)
}

fn create_ob_struct(url: String) -> OrderBook {
    let book_object = create_json_object(url).unwrap();
    let balance: f32 = 10000.0;

    let mut v_asks: Vec<[f32; 2]> = Vec::new();
    for i in 0..50 {
        let price: f32 = book_object.get("asks").unwrap().get(i).unwrap().get(0).unwrap().as_str().unwrap().parse().unwrap();
        let vol: f32 = book_object.get("asks").unwrap().get(i).unwrap().get(1).unwrap().as_str().unwrap().parse().unwrap();
        v_asks.push([price, vol]);
        println!("{:?}", v_asks[i]);
    }

    println!();
    println!();

    let mut v_bids: Vec<[f32; 2]> = Vec::new();
    for j in 0..50 {
        let price: f32 = book_object.get("bids").unwrap().get(j).unwrap().get(0).unwrap().as_str().unwrap().parse().unwrap();
        let vol: f32 = book_object.get("bids").unwrap().get(j).unwrap().get(1).unwrap().as_str().unwrap().parse().unwrap();
        v_bids.push([price, vol]);
        println!("{:?}", v_bids[j]);
    }

    let mut btc_usdt_ob = OrderBook {
        asks: v_asks,
        bids: v_bids,
        balance: balance
    };

    return btc_usdt_ob;
}
