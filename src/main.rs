use std::env;

use hotel_reservation::controller::hotel_reservation;
use hotel_reservation::domain::hotel::Hotel;

fn main() {
    let hotels = vec![Hotel::lake_inn(), Hotel::falls_inn(), Hotel::forest_inn()];
    let args: Vec<String> = env::args().collect();

    if let Some(input) = args.get(1) {
        match hotel_reservation(input, &hotels) {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err),
        }
    } else {
        println!("Please provide an input in format: <Regular|Rewards>: <date_1>, <date_2>, <date_3>, ...");
        println!("Example: Regular: 16Mar2009(mon), 17Mar2009(tue), 18Mar2009(wed)");
    }
}
