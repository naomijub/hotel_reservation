use std::ops::RangeInclusive;

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

use super::{input::CustomerType, quote::Quote};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Rates {
    weekday: u32,
    weekend: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotel {
    name: String,
    rating: u8,
    regular: Rates,
    rewards: Rates,
    blackout: Option<RangeInclusive<NaiveDate>>,
}

impl Hotel {
    pub fn lake_inn() -> Self {
        Self::new("Lake Inn".to_string(), 3, (110, 90), (80, 80), None)
    }

    pub fn falls_inn() -> Self {
        Self::new(
            "Falls Inn".to_string(),
            4,
            (160, 60),
            (110, 50),
            Some(
                NaiveDate::from_ymd_opt(2009, 12, 23).unwrap()
                    ..=NaiveDate::from_ymd_opt(2010, 1, 3).unwrap(),
            ),
        )
    }

    pub fn forest_inn() -> Self {
        Self::new(
            "Forest Inn".to_string(),
            5,
            (220, 150),
            (100, 40),
            Some(
                NaiveDate::from_ymd_opt(2009, 7, 1).unwrap()
                    ..=NaiveDate::from_ymd_opt(2009, 9, 30).unwrap(),
            ),
        )
    }

    pub fn new(
        name: String,
        rating: u8,
        regular: (u32, u32),
        rewards: (u32, u32),
        blackout: Option<RangeInclusive<NaiveDate>>,
    ) -> Self {
        Self {
            name,
            rating,
            regular: regular.into(),
            rewards: rewards.into(),
            blackout,
        }
    }

    pub fn quote(&self, customer_type: CustomerType, dates: Vec<NaiveDate>) -> Quote {
        let blackout = self.blackout.as_ref();
        let total = dates
            .iter()
            .map(|date| {
                match (
                    blackout.map_or(false, |range| range.contains(date)),
                    customer_type,
                    date.weekday(),
                ) {
                    (
                        true,
                        _,
                        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri,
                    ) => self.regular.weekday,
                    (true, _, Weekday::Sat | Weekday::Sun) => self.regular.weekend,
                    (
                        false,
                        CustomerType::Regular,
                        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri,
                    ) => self.regular.weekday,
                    (false, CustomerType::Regular, Weekday::Sat | Weekday::Sun) => {
                        self.regular.weekend
                    }
                    (
                        false,
                        CustomerType::Rewards,
                        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri,
                    ) => self.rewards.weekday,
                    (false, CustomerType::Rewards, Weekday::Sat | Weekday::Sun) => {
                        self.rewards.weekend
                    }
                }
            })
            .sum();

        Quote {
            total,
            rating: self.rating,
            name: self.name.clone(),
        }
    }
}

impl From<(u32, u32)> for Rates {
    fn from(value: (u32, u32)) -> Self {
        Self {
            weekday: value.0,
            weekend: value.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates_from_tuple() {
        let rates = Rates::from((10, 20));
        assert_eq!(rates.weekday, 10);
        assert_eq!(rates.weekend, 20);
    }

    #[test]
    fn quote_lake_inn_regular() {
        let quote = Hotel::lake_inn().quote(
            CustomerType::Regular,
            vec![
                NaiveDate::from_ymd_opt(2009, 3, 20).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 21).unwrap(),
            ],
        );
        assert_eq!(quote.total, 200);
        assert_eq!(quote.rating, 3);
    }

    #[test]
    fn quote_falls_inn_rewards() {
        let quote = Hotel::falls_inn().quote(
            CustomerType::Rewards,
            vec![
                NaiveDate::from_ymd_opt(2009, 3, 20).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 21).unwrap(),
            ],
        );
        assert_eq!(quote.total, 160);
        assert_eq!(quote.rating, 4);
    }

    #[test]
    fn quote_forest_inn_regular() {
        let quote = Hotel::forest_inn().quote(
            CustomerType::Regular,
            vec![
                NaiveDate::from_ymd_opt(2009, 3, 20).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 21).unwrap(),
            ],
        );
        assert_eq!(quote.total, 370);
        assert_eq!(quote.rating, 5);
    }

    #[test]
    fn quote_falls_inn_rewards_a_few_days_from_holidays() {
        let quote = Hotel::falls_inn().quote(
            CustomerType::Rewards,
            vec![
                NaiveDate::from_ymd_opt(2009, 12, 22).unwrap(),
                NaiveDate::from_ymd_opt(2009, 12, 23).unwrap(),
            ],
        );
        assert_eq!(quote.total, 270);
        assert_eq!(quote.rating, 4);
    }

    #[test]
    fn quote_forest_inn_rewards_summer() {
        let quote = Hotel::forest_inn().quote(
            CustomerType::Rewards,
            vec![
                NaiveDate::from_ymd_opt(2009, 8, 22).unwrap(),
                NaiveDate::from_ymd_opt(2009, 8, 23).unwrap(),
            ],
        );
        assert_eq!(quote.total, 300);
        assert_eq!(quote.rating, 5);
    }

    #[test]
    fn quote_forest_inn_reegular_summer() {
        let quote = Hotel::forest_inn().quote(
            CustomerType::Regular,
            vec![
                NaiveDate::from_ymd_opt(2009, 8, 22).unwrap(),
                NaiveDate::from_ymd_opt(2009, 8, 23).unwrap(),
            ],
        );
        assert_eq!(quote.total, 300);
        assert_eq!(quote.rating, 5);
    }
}
