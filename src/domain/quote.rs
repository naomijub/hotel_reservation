#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote {
    pub total: u32,
    pub rating: u8,
    pub name: String,
}

impl PartialOrd for Quote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for Quote {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total
            .cmp(&other.total)
            .then_with(|| other.rating.cmp(&self.rating))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_price_first() {
        let quote1 = Quote {
            total: 100,
            rating: 3,
            name: "Lake Inn".to_string(),
        };
        let quote2 = Quote {
            total: 200,
            rating: 4,
            name: "Falls Inn".to_string(),
        };
        assert!(quote1 < quote2);
    }

    #[test]
    fn same_price_higher_rating_first() {
        let quote1 = Quote {
            total: 100,
            rating: 3,
            name: "Lake Inn".to_string(),
        };
        let quote2 = Quote {
            total: 100,
            rating: 5,
            name: "Falls Inn".to_string(),
        };
        assert!(quote2 < quote1);
    }
}
