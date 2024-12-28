use crate::domain::{hotel::Hotel, input::Input, quote::Quote};

pub fn cheapest_hotel(hotels: &[Hotel], input_data: Input) -> String {
    let mut quotes = hotels
        .iter()
        .map(|hotel| hotel.quote(input_data.customer_type, input_data.dates.clone()))
        .collect::<Vec<Quote>>();
    quotes.sort();
    quotes.first().unwrap().name.clone()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn hotel_case_1() {
        let input =
            Input::from_str("Regular: 16Mar2009(mon), 17Mar2009(tue), 18Mar2009(wed)").unwrap();
        let hotel = hotel_list();

        let cheapest = cheapest_hotel(&hotel, input);
        assert_eq!(cheapest, "Lake Inn");
    }

    #[test]
    fn hotel_case_2() {
        let input =
            Input::from_str("regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)").unwrap();
        let hotel = hotel_list();

        let cheapest = cheapest_hotel(&hotel, input);
        assert_eq!(cheapest, "Falls Inn");
    }

    #[test]
    fn hotel_case_3() {
        let input =
            Input::from_str("Rewards: 26Mar2009(thu), 27Mar2009(fri), 28Mar2009(sat)").unwrap();
        let hotel = hotel_list();

        let cheapest = cheapest_hotel(&hotel, input);
        assert_eq!(cheapest, "Forest Inn");
    }

    fn hotel_list() -> Vec<Hotel> {
        vec![Hotel::lake_inn(), Hotel::falls_inn(), Hotel::forest_inn()]
    }
}
