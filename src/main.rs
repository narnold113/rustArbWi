extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde_json::Value;

#[derive(Debug)]
struct Order {
  price: f64,
  volume: f64,
}

#[derive(Debug)]
struct OrderBook {
  asks: Vec<Order>,
  bids: Vec<Order>,
  balance: f64,
}

impl OrderBook {
  fn get_asks_wp(&self) -> f64 {
    self.get_wp(&self.asks)
  }

  fn get_bids_wp(&self) -> f64 {
    self.get_wp(&self.bids)
  }

  fn get_wp(&self, orders: &[Order]) -> f64 {
    let mut volume = 0.0;
    let mut w_price = 0.0;

    for order in orders {
      volume += order.price * order.volume;
      w_price += order.price * ((order.price * order.volume) / self.balance);
      if volume > self.balance {
        let remainder = volume - self.balance;
        w_price -= order.price * (remainder / self.balance);
        break;
      }
    }
    return w_price;
  }
}

fn main() {
  let btc_usdt_ob = create_ob_struct("https://api.binance.com/api/v1/depth?symbol=BTCUSDT&limit=50");
  let btc_usdt_ask: f64 = btc_usdt_ob.get_asks_wp();
  let btc_usdt_bid: f64 = btc_usdt_ob.get_bids_wp();

  println!("{:?}", btc_usdt_bid);
}

fn get_order(url: &str) -> String {
  reqwest::get(url)
    .expect("Couldn't make request.")
    .text()
    .expect("Couldn't read response text")
}

fn create_json_object(url: &str) -> Result<Value, ()> {
  let book = get_order(url);
  let book_parsed: Value = serde_json::from_str(&book).unwrap();
  Ok(book_parsed)
}

fn create_ob_struct(url: &str) -> OrderBook {
  let book_object = create_json_object(url).unwrap();
  let balance: f64 = 10000.0;

  fn parse_struct(value: &Value) -> Vec<Order> {
    let mut v: Vec<Order> = Vec::new();
    let mut i = 0;
    loop {
      if let Some(order) = value.get(i) {
        let price: f64 = order.get(0).unwrap().as_str().unwrap().parse().unwrap();
        let volume: f64 = order.get(1).unwrap().as_str().unwrap().parse().unwrap();
        let order = Order { price, volume };
        println!("{:?}", &order);
        v.push(order);
      } else {
        return v;
      }
      i += 1;
    }
  }

  let asks = book_object.get("asks").expect("No asks key");
  let asks = parse_struct(asks);

  println!();
  println!();

  let bids = book_object.get("asks").expect("No bids key");
  let bids = parse_struct(bids);

  let mut btc_usdt_ob = OrderBook { asks, bids, balance };

  return btc_usdt_ob;
}
