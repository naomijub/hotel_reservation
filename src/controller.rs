use std::{borrow::Cow, str::FromStr};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tower::BoxError;

use crate::{
    db::SharedState,
    domain::{
        hotel::Hotel,
        input::{Input, InputParseError},
    },
    service::cheapest_hotel,
};

pub async fn list_handle(State(state): State<SharedState>) -> (StatusCode, Json<Vec<Hotel>>) {
    let db = state.read().unwrap();

    (StatusCode::OK, Json(db.list()))
}

pub async fn cheapest_handle(
    State(state): State<SharedState>,
    Json(payload): Json<String>,
) -> (StatusCode, Json<String>) {
    let hotels = state.read().unwrap().list();

    (
        StatusCode::OK,
        Json(hotel_reservation(&payload, &hotels).unwrap()),
    )
}

pub async fn add_handle(
    State(state): State<SharedState>,
    Json(payload): Json<Hotel>,
) -> (StatusCode, Json<String>) {
    state.write().unwrap().add(payload);

    (StatusCode::OK, Json("Hotel added".to_string()))
}

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}

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
