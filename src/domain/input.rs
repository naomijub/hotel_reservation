use std::str::FromStr;

use chrono::prelude::*;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InputParseError {
    #[error("Invalid customer input: {0}")]
    InvalidCustomerInput(String),
    #[error("Invalid date input: {0}")]
    InvalidDateInput(String),
    #[error("Invalid input format: {0}")]
    InvalidInputFormat(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomerType {
    Regular,
    Rewards,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub customer_type: CustomerType,
    pub dates: Vec<NaiveDate>,
}

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (customer_type, dates) = s
            .split_once(':')
            .ok_or_else(|| InputParseError::InvalidInputFormat(s.to_string()))?;
        let customer_type = match customer_type.to_lowercase().as_str() {
            "regular" => CustomerType::Regular,
            "rewards" => CustomerType::Rewards,
            _ => {
                return Err(InputParseError::InvalidCustomerInput(
                    customer_type.to_string(),
                ))
            }
        };
        let dates = dates
            .replace("tues", "tue")
            .replace("thur", "thu")
            .split(',')
            .map(|s| {
                NaiveDate::parse_from_str(s.trim(), "%d%b%Y(%a)")
                    .map_err(|_| InputParseError::InvalidDateInput(s.trim().to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            customer_type,
            dates,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_regular_3_dates() {
        let input = "Regular: 16Mar2009(mon), 17Mar2009(tue), 18Mar2009(wed)";
        let input = Input::from_str(input).unwrap();
        assert_eq!(input.customer_type, CustomerType::Regular);
        assert_eq!(
            input.dates,
            vec![
                NaiveDate::from_ymd_opt(2009, 3, 16).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 17).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 18).unwrap()
            ]
        );
    }

    #[test]
    fn input_lowercase() {
        let input = "regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)";
        let input = Input::from_str(input).unwrap();
        assert_eq!(input.customer_type, CustomerType::Regular);
        assert_eq!(
            input.dates,
            vec![
                NaiveDate::from_ymd_opt(2009, 3, 20).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 21).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 22).unwrap()
            ]
        );
    }

    #[test]
    fn input_rewards_3_dates() {
        let input = "Rewards: 26Mar2009(thu), 27Mar2009(fri), 28Mar2009(sat)";
        let input = Input::from_str(input).unwrap();
        assert_eq!(input.customer_type, CustomerType::Rewards);
        assert_eq!(
            input.dates,
            vec![
                NaiveDate::from_ymd_opt(2009, 3, 26).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 27).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 28).unwrap()
            ]
        );
    }

    #[test]
    fn input_regular_3_dates_4_letters() {
        let input = "Regular: 17Mar2009(tues), 18Mar2009(wed), 19Mar2009(thur)";
        let input = Input::from_str(input).unwrap();
        assert_eq!(input.customer_type, CustomerType::Regular);
        assert_eq!(
            input.dates,
            vec![
                NaiveDate::from_ymd_opt(2009, 3, 17).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 18).unwrap(),
                NaiveDate::from_ymd_opt(2009, 3, 19).unwrap(),
            ]
        );
    }

    #[test]
    fn unknown_customer() {
        let input = "Unknown: 17Mar2009(tues), 18Mar2009(wed), 19Mar2009(thur)";
        let err = Input::from_str(input).unwrap_err();

        assert_eq!(
            err,
            InputParseError::InvalidCustomerInput("Unknown".to_string())
        );
    }

    #[test]
    fn unknown_format() {
        let input = "Regular => 17Mar2009(tues), 18Mar2009(wed), 19Mar2009(thur)";
        let err = Input::from_str(input).unwrap_err();

        assert_eq!(err, InputParseError::InvalidInputFormat(input.to_string()));
    }

    #[test]
    fn invalid_date() {
        let input = "Regular: 32Mar2009(tue), 18Mar2009(wed), 19Mar2009(thur)";
        let err = Input::from_str(input).unwrap_err();

        assert_eq!(
            err,
            InputParseError::InvalidDateInput("32Mar2009(tue)".to_string())
        );
    }
}
