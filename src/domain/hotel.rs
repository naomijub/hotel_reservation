use chrono::{Datelike, NaiveDate, Weekday};

use super::{input::CustomerType, quote::Quote};

struct Rates {
    weekday: u32,
    weekend: u32,
}

pub struct Hotel {
    name: String,
    rating: u8,
    regular: Rates,
    rewards: Rates,
}

impl Hotel {
    pub fn lake_inn() -> Self {
        Self::new("Lake Inn".to_string(), 3, (110, 90), (80, 80))
    }

    pub fn falls_inn() -> Self {
        Self::new("Falls Inn".to_string(), 4, (160, 60), (110, 50))
    }

    pub fn forest_inn() -> Self {
        Self::new("Forest Inn".to_string(), 5, (220, 150), (100, 40))
    }

    pub fn new(name: String, rating: u8, regular: (u32, u32), rewards: (u32, u32)) -> Self {
        Self {
            name,
            rating,
            regular: regular.into(),
            rewards: rewards.into(),
        }
    }

    pub fn quote(&self, customer_type: CustomerType, dates: Vec<NaiveDate>) -> Quote {
        let rates = match customer_type {
            CustomerType::Regular => &self.regular,
            CustomerType::Rewards => &self.rewards,
        };
        let mut total = 0;
        for date in dates {
            total += match date.weekday() {
                Weekday::Sat | Weekday::Sun => rates.weekend,
                _ => rates.weekday,
            }
        }
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
}
