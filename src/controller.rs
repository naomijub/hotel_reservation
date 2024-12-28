use std::str::FromStr;

use crate::{
    domain::{
        hotel::Hotel,
        input::{Input, InputParseError},
    },
    service::cheapest_hotel,
};

pub fn hotel_reservation(input_data: &str, hotels: &[Hotel]) -> Result<String, InputParseError> {
    let input = Input::from_str(input_data)?;
    Ok(cheapest_hotel(hotels, input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotel_case_1() {
        let input = "Regular: 16Mar2009(mon), 17Mar2009(tue), 18Mar2009(wed)";
        let hotels = vec![Hotel::lake_inn(), Hotel::falls_inn(), Hotel::forest_inn()];
        let output = hotel_reservation(input, &hotels).unwrap();
        assert_eq!(output, "Lake Inn");
    }
}
